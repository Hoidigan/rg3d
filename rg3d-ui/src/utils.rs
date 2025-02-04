use crate::{
    core::algebra::Vector2,
    core::pool::Handle,
    vector_image::{Primitive, VectorImageBuilder},
    widget::WidgetBuilder,
    BuildContext, HorizontalAlignment, UiNode, VerticalAlignment, BRUSH_BRIGHT,
};

pub enum ArrowDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn make_arrow(
    ctx: &mut BuildContext,
    orientation: ArrowDirection,
    size: f32,
) -> Handle<UiNode> {
    VectorImageBuilder::new(
        WidgetBuilder::new()
            .with_foreground(BRUSH_BRIGHT)
            .with_horizontal_alignment(HorizontalAlignment::Center)
            .with_vertical_alignment(VerticalAlignment::Center),
    )
    .with_primitives(vec![match orientation {
        ArrowDirection::Top => Primitive::Triangle {
            points: [
                Vector2::new(size * 0.5, 0.0),
                Vector2::new(size, size),
                Vector2::new(0.0, size),
            ],
        },
        ArrowDirection::Bottom => Primitive::Triangle {
            points: [
                Vector2::new(0.0, 0.0),
                Vector2::new(size, 0.0),
                Vector2::new(size * 0.5, size),
            ],
        },
        ArrowDirection::Right => Primitive::Triangle {
            points: [
                Vector2::new(0.0, 0.0),
                Vector2::new(size, size * 0.5),
                Vector2::new(0.0, size),
            ],
        },
        ArrowDirection::Left => Primitive::Triangle {
            points: [
                Vector2::new(0.0, size * 0.5),
                Vector2::new(size, 0.0),
                Vector2::new(size, size),
            ],
        },
    }])
    .build(ctx)
}
