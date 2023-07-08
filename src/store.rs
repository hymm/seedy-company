use crate::game_state::StoreSetupState;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct StorePlugin;
impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_pedestals
                .in_schedule(OnEnter(StoreSetupState::PedestalSelect))
                .run_if(|q: Query<&ItemDisplay>| q.is_empty()),
        )
        .add_system(handle_pedestal_click.run_if(in_state(StoreSetupState::PedestalSelect)));
    }
}

#[derive(Component)]
pub struct ItemDisplay;
impl ItemDisplay {
    const SIZE: f32 = 24.;
    fn point_inside(point: Vec2, transform: &Transform) -> bool {
        let center = transform.translation.truncate();
        point.x > center.x - Self::SIZE / 2.
            && point.x < center.x + Self::SIZE / 2.
            && point.y > center.y - Self::SIZE / 2.
            && point.y < center.y + Self::SIZE / 2.
    }
}

#[derive(Resource)]
pub struct SelectedPedestal(pub Entity);

fn spawn_pedestals(mut commands: Commands) {
    for i in 0..3 {
        commands.spawn((
            ItemDisplay,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(ItemDisplay::SIZE, ItemDisplay::SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(i as f32 * 48.0, 0., 0.)),
                ..default()
            },
        ));
    }
}

fn handle_pedestal_click(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    displays: Query<(Entity, &Transform), With<ItemDisplay>>,
    mut state: ResMut<NextState<StoreSetupState>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        let window = window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            for (e, display_transform) in &displays {
                if ItemDisplay::point_inside(world_position, display_transform) {
                    commands.insert_resource(SelectedPedestal(e));
                    state.set(StoreSetupState::Inventory);
                }
            }
        }
    }
}
