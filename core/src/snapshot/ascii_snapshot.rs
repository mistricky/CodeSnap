use std::cmp::max;

use crate::utils::code::{calc_max_line_number_length, calc_wh, prepare_code};

use arboard::Clipboard;

const SPACE_BOTH_SIDE: usize = 2;

pub struct ASCIISnapshot {
    content: String,
}

fn optional(component: String, is_view: bool) -> String {
    if is_view {
        component
    } else {
        "".to_string()
    }
}

impl ASCIISnapshot {
    pub fn copy(&self) -> anyhow::Result<()> {
        Clipboard::new()?.set_text(self.content.as_str())?;

        Ok(())
    }

    pub fn save(&self, _save_path: &str) -> anyhow::Result<()> {
        todo!()
    }

    pub fn from_config(config: crate::config::SnapshotConfig) -> anyhow::Result<Self> {
        let code = prepare_code(&config.code.content);
        let (width, height) = calc_wh(&code, 1., 1.);
        let calc_line_number_width = |start_line_number: u32| {
            calc_max_line_number_length(height as usize, start_line_number)
        };
        let len = config
            .code
            .clone()
            .file_path
            .and_then(|x| Some(x.len()))
            .unwrap_or(0);
        let frame_width = max(width as usize, len + SPACE_BOTH_SIDE);
        let frame_width = match config.code.line_number {
            Some(ref line_number) => {
                frame_width + SPACE_BOTH_SIDE + calc_line_number_width(line_number.start_number)
            }
            None => frame_width,
        };
        let line = format!("│{}│\n", "─".repeat(frame_width));
        let frame_width_with_content = frame_width - 1;
        let top_frame = format!("╭{}╮\n", "─".repeat(frame_width));
        let bottom_frame = format!("╰{}╯", "─".repeat(frame_width));
        let code = code
            .lines()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "│ {:1$} │\n",
                    match config.code.line_number {
                        Some(ref line_number) => format!(
                            "{:1$} {line}",
                            line_number.start_number as usize + i,
                            calc_line_number_width(line_number.start_number),
                        ),
                        None => line.to_string(),
                    },
                    frame_width_with_content - 1
                )
            })
            .collect::<String>();
        let text_line = |text: &str| format!("│ {:1$}│\n", text, frame_width_with_content);
        let breadcrumbs = optional(
            format!(
                "{}{line}",
                text_line(&config.code.file_path.unwrap_or(String::from("")))
            ),
            config.code.breadcrumbs.is_some(),
        );
        let ascii_snapshot = format!("{top_frame}{breadcrumbs}{code}{bottom_frame}");

        Ok(ASCIISnapshot {
            content: ascii_snapshot,
        })
    }
}
