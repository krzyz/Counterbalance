use bevy::{prelude::*, render::view::RenderLayers};
use bevy_prototype_lyon::prelude::*;

use crate::{
    battle::{battle_field::BattleField, init::get_scaling},
    character::{Attribute, AttributeType, Attributes},
};

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_bar);
        app.add_system(update_bar);
        app.add_system(correct_bar_position);
    }
}

#[derive(Component)]
pub struct Bar {
    attribute: AttributeType,
}

impl Bar {
    pub fn new(attribute: AttributeType) -> Bar {
        Self { attribute }
    }
}

fn position_above_image(image: &Image) -> Transform {
    Transform::from_xyz(0.0, 0.5 * image.size().y + 20.0, 0.0)
}

fn get_inner_shape(bar: &Bar, attributes: &Attributes, border_size: f32) -> shapes::Rectangle {
    let attribute = attributes
        .0
        .get(&bar.attribute)
        .expect("Missing attribute to which bar corresponds");

    match *attribute {
        Attribute::Value(_) => panic!("Bar not supported for \"value\" attribute!"),
        Attribute::Gauge { value, min, max } => shapes::Rectangle {
            extents: Vec2::new(
                (80.0 - border_size) * (value - min).clamp(min, max) as f32 / (max - min) as f32,
                -(20.0 - border_size),
            ),
            origin: RectangleOrigin::TopLeft,
        },
    }
}

fn setup_bar(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    query: Query<
        (
            Entity,
            &Bar,
            &Attributes,
            &Handle<Image>,
            Option<&RenderLayers>,
        ),
        Added<Bar>,
    >,
) {
    const BORDER_SIZE: f32 = 2.0;

    let shape = shapes::Rectangle {
        extents: Vec2::new(80.0, 20.0),
        ..default()
    };

    for (entity, bar, attributes, image_handle, render_layers) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            let (visibility, transform) = images
                .get(image_handle)
                .map(|image| (Visibility::default(), position_above_image(image)))
                .unwrap_or_else(|| (Visibility::Hidden, Transform::default()));

            let mut bar_entity = parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform,
                    visibility,
                    ..default()
                },
                Fill::color(Color::WHITE),
                Stroke::new(Color::BLACK, 2.0),
            ));

            bar_entity.with_children(|parent| {
                let inner_shape = get_inner_shape(bar, attributes, BORDER_SIZE);

                let mut bar_inside = parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&inner_shape),
                        transform: Transform::from_xyz(
                            -0.5 * (shape.extents.x - BORDER_SIZE),
                            -0.5 * (shape.extents.y - BORDER_SIZE),
                            1.0,
                        ),
                        ..default()
                    },
                    Fill::color(Color::GREEN),
                ));

                if let Some(layers) = render_layers {
                    bar_inside.insert(*layers);
                }
            });

            if let Some(layers) = render_layers {
                bar_entity.insert(*layers);
            }
        });
    }
}

fn invert_scale(scale: &Vec3) -> Vec3 {
    Vec3::new(1.0 / scale.x, 1.0 / scale.y, 1.0 / scale.z)
}

fn correct_bar_position(
    images: Res<Assets<Image>>,
    battle_field: Option<Res<BattleField>>,
    mut ev_image_asset: EventReader<AssetEvent<Image>>,
    entity_query: Query<(&Handle<Image>, &Children), With<Bar>>,
    mut transform_query: Query<(&mut Transform, &mut Visibility), Without<Bar>>,
) {
    for ev in ev_image_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            for children in entity_query
                .iter()
                .filter_map(|(q_handle, children)| (q_handle == handle).then_some(children))
            {
                let transform_entity = *children.first().expect("Expected bar to have a child");
                let (mut transform, mut visibility) = transform_query
                    .get_mut(transform_entity)
                    .expect("Missing transform");

                *transform = position_above_image(
                    images.get(handle).expect("Wrong image asset created event"),
                );

                if let Some(battle_field) = battle_field.as_ref() {
                    *transform = transform.with_scale(invert_scale(&get_scaling(
                        images.get(handle),
                        battle_field.tile_size(),
                    )));
                }
                *visibility = Visibility::default();
            }
        }
    }
}

fn update_bar(
    bar_query: Query<(&Bar, &Attributes, &Children), Changed<Attributes>>,
    stroke_query: Query<(&Stroke, &Children)>,
    mut inner_bar_query: Query<&mut Path>,
) {
    for (bar, attributes, children) in bar_query.iter() {
        let stroke_entity = *children.first().expect("Expected bar to have a child");
        if let Ok((stroke, stroke_children)) = stroke_query.get(stroke_entity) {
            let inner_bar_entity = stroke_children
                .first()
                .expect("Expected stroke to have a child");

            let mut inner_bar = inner_bar_query
                .get_mut(*inner_bar_entity)
                .expect("Mismatched child");

            let border_size = stroke.options.line_width;

            let inner_shape = get_inner_shape(bar, attributes, border_size);

            *inner_bar = GeometryBuilder::build_as(&inner_shape);
        }
    }
}
