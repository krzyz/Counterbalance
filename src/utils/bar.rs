use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bar);
    }
}

#[derive(Component)]
pub struct Bar {
    min: f32,
    max: f32,
    value: f32,
}

impl Bar {
    pub fn new(min: f32, max: f32, value: f32) -> Bar {
        Self { min, max, value }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

pub fn spawn_bar(commands: &mut Commands) -> Entity {
    const BORDER_SIZE: f32 = 2.0;

    let shape = shapes::Rectangle {
        extents: Vec2::new(80.0, 20.0),
        ..default()
    };

    let inner_shape = shapes::Rectangle {
        extents: Vec2::new(80.0 - BORDER_SIZE, -(20.0 - BORDER_SIZE)),
        origin: RectangleOrigin::TopLeft,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::BLACK, 2.0),
        ))
        .with_children(|parent| {
            parent.spawn((
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
        })
        .id()
}

fn update_bar(
    bar_query: Query<(&Bar, &Children, &Stroke), Changed<Bar>>,
    mut inner_bar_query: Query<&mut Path>,
) {
    for (bar, children, stroke) in bar_query.iter() {
        let inner_bar_entity = children.first().expect("Expected bar to have a child");

        let mut inner_bar = inner_bar_query
            .get_mut(*inner_bar_entity)
            .expect("Mismatched child");

        let border_size = stroke.options.line_width;

        let inner_shape = shapes::Rectangle {
            extents: Vec2::new(
                (80.0 - border_size) * (bar.value - bar.min) / (bar.max - bar.min),
                -(20.0 - border_size),
            ),
            origin: RectangleOrigin::TopLeft,
        };

        *inner_bar = GeometryBuilder::build_as(&inner_shape);
    }
}
