use super::*;

fn get_children_string_vec(node: &LinkedNode, ctx: &mut Ctx) -> Vec<String> {
    let mut inner_res: Vec<String> = vec![];
    for child in node.children() {
        let child_fmt = visit(&child, ctx);
        inner_res.push(child_fmt);
    }
    inner_res
}

/// only format tight cause it would not be supported to format breaking in code blocks
///
/// it could be supported in parenthesized.
#[instrument(skip_all, ret)]
pub(crate) fn format_module_import(
    parent: &LinkedNode,
    children: &[String],
    ctx: &mut Ctx,
) -> String {
    let res = format_module_import_tight(parent, children, ctx);

    if crate::utils::max_line_length(&res) >= ctx.config.max_line_length {
        return format_module_import_breaking(parent, children, ctx);
    }
    res
}

#[derive(Debug, Default)]
struct Info {
    has_left_paren: bool,
    has_right_paren: bool,
}

/// not integrated and never used for now
#[instrument(skip_all)]
pub(crate) fn format_module_import_breaking(
    parent: &LinkedNode,
    children: &[String],
    ctx: &mut Ctx,
) -> String {
    // dbg!("inside format_import_items_breaking()");
    let mut res = String::new();
    let mut info = Info::default();
    for node in parent.children() {
        match node.kind() {
            LeftParen => {
                info.has_left_paren = true;
            }
            RightParen => {
                info.has_right_paren = true;
            }
            _ => {}
        }
    }
    for (s, node) in children.iter().zip(parent.children()) {
        // dbg!(s, &node);
        match node.kind() {
            _ if ctx.off => res.push_str(node.text()), // ?
            Space if node.prev_sibling().is_some_and(|s| s.kind() == LeftParen) => {}
            Space if node.next_sibling().is_some_and(|s| s.kind() == RightParen) => {}
            ImportItems => {
                let children = get_children_string_vec(&node, ctx);
                let s = format_import_items_breaking(&node, &children, ctx, &info);
                ctx.push_raw_in(&s, &mut res);
            }
            // LineComment | BlockComment => {
            //     ctx.push_in(" ", &mut res);
            //     ctx.push_raw_in(s, &mut res);
            //     ctx.push_in("\n", &mut res);
            //     ctx.push_raw_in(&ctx.get_indent(), &mut res);
            //     ctx.just_spaced = true;
            // }
            _ => {
                ctx.push_raw_in(s, &mut res);
            }
        }
    }
    res
}

/// not integrated and never used for now
#[instrument(skip_all)]
pub(crate) fn format_import_items_breaking(
    parent: &LinkedNode,
    children: &[String],
    ctx: &mut Ctx,
    Info {
        has_left_paren,
        has_right_paren,
    }: &Info,
) -> String {
    // dbg!("inside format_import_items_breaking()");
    let mut res = String::new();
    for (s, node) in children.iter().zip(parent.children()) {
        // dbg!(s, &node);
        match node.kind() {
            _ if ctx.off => res.push_str(node.text()), // ?
            ImportItemPath => {
                let is_first = !utils::prev_is_ignoring(
                    &node,
                    ImportItemPath,
                    &[Space, Comma, LineComment, BlockComment],
                );
                let is_last = !utils::next_is_ignoring(
                    &node,
                    ImportItemPath,
                    &[Space, Comma, LineComment, BlockComment],
                );
                if is_first {
                    if !has_left_paren {
                        ctx.push_in("(", &mut res);
                    }
                    ctx.push_in("\n", &mut res);
                }
                ctx.push_raw_in(&ctx.get_indent(), &mut res);
                ctx.push_raw_in(s, &mut res);
                ctx.push_in(",", &mut res);
                if is_last {
                    ctx.push_in("\n", &mut res);
                    if !has_right_paren {
                        ctx.push_in(")", &mut res);
                    }
                } else {
                    ctx.push_in("\n", &mut res);
                }
            }
            Comma => {}
            Space => {}
            LineComment | BlockComment => {
                ctx.push_raw_indent(s, &mut res);
                ctx.push_in("\n", &mut res);
            }
            _ => {
                ctx.push_in(s, &mut res);
            }
        }
    }
    res
}

#[instrument(skip_all)]
pub(crate) fn format_module_import_tight(
    parent: &LinkedNode,
    children: &[String],
    ctx: &mut Ctx,
) -> String {
    // dbg!("inside format_module_import_tight()");
    let mut res = String::new();
    for (s, node) in children.iter().zip(parent.children()) {
        // dbg!(s, &node);
        match node.kind() {
            _ if ctx.off => res.push_str(node.text()), // ?
            LeftParen => {}
            Space if node.prev_sibling().is_some_and(|s| s.kind() == LeftParen) => {}
            Space if node.next_sibling().is_some_and(|s| s.kind() == RightParen) => {}
            RightParen => {}
            ImportItems => {
                let mut inner_res: Vec<String> = vec![];
                for child in node.children() {
                    let child_fmt = visit(&child, ctx);
                    inner_res.push(child_fmt);
                }
                let result = format_import_items_tight(&node, &inner_res, ctx);
                ctx.push_raw_in(&result, &mut res);
            }
            // LineComment | BlockComment => {
            //     ctx.push_in(" ", &mut res);
            //     ctx.push_raw_in(s, &mut res);
            //     ctx.push_in("\n", &mut res);
            //     ctx.push_raw_in(&ctx.get_indent(), &mut res);
            //     ctx.just_spaced = true;
            // }
            _ => {
                ctx.push_raw_in(s, &mut res);
            }
        }
    }
    res
}

#[instrument(skip_all)]
pub(crate) fn format_import_items_tight(
    parent: &LinkedNode,
    children: &[String],
    ctx: &mut Ctx,
) -> String {
    // dbg!("inside format_import_items_tight()");
    let mut res = String::new();
    for (s, node) in children.iter().zip(parent.children()) {
        // dbg!(s, &node);
        match node.kind() {
            _ if ctx.off => res.push_str(node.text()), // ?

            ImportItemPath => {
                let is_last = !utils::next_is_ignoring(
                    &node,
                    ImportItemPath,
                    &[Space, Comma, LineComment, BlockComment],
                );
                ctx.push_raw_in(s, &mut res);
                if !is_last {
                    ctx.push_in(",", &mut res);
                    ctx.push_in(" ", &mut res);
                }
            }
            Comma => {}
            Space => {}
            // LineComment | BlockComment => {
            //     ctx.push_in(" ", &mut res);
            //     ctx.push_raw_in(s, &mut res);
            //     ctx.push_in("\n", &mut res);
            //     ctx.push_raw_in(&ctx.get_indent(), &mut res);
            //     ctx.just_spaced = true;
            // }
            _ => {
                ctx.push_raw_in(s, &mut res);
            }
        }
    }
    res
}
