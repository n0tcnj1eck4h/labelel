use super::project::Project;
use crate::colors::COLORS;
use crate::project::Label;
use crate::project::Segment;
use crate::project::Tool;
use egui::Color32;
use egui::FontId;
use egui::Rangef;
use egui::Sense;
use egui::TextBuffer;
use egui_extras::Column;
use std::path::PathBuf;

pub struct SegmentDrag {
    segment_index: usize,
    awesome: egui::Vec2,
    icon: egui::CursorIcon,
}

#[derive(Copy, Clone)]
pub struct Input {
    left: bool,
    right: bool,
    accept: bool,
    delete: bool,
    clone: bool,
    tool: Option<Tool>,
    hover_pos: Option<egui::Pos2>,
    scroll_delta: egui::Vec2,
}

pub struct App {
    pub project: Option<Project>,
    pub smooth_scroll: bool,
    pub advance_on_accept: bool,
    pub message_box: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: Default::default(),
            smooth_scroll: true,
            advance_on_accept: false,
            message_box: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let input = self.read_inputs(ctx);

        if let Some(project) = &mut self.project {
            if let Some(tool) = input.tool {
                project.tool = tool;
            }

            if input.left {
                project.back();
            }

            if input.right {
                project.advance();
            }
        }

        self.sidebar(ctx);
        self.central_panel(ctx, input);
        self.timeline(ctx);
        if let Some(project) = &mut self.project
            && let Some((id, name)) = &mut project.add_label_modal
        {
            let mut modal = egui::Modal::new("addlabelbox".into());
            modal.area = modal.area.default_size((128.0, 100.0));
            let clicked = modal
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add(egui::TextEdit::singleline(name));
                        ui.columns(2, |uis| {
                            if uis[0].button("").clicked() {
                                Some(true)
                            } else if uis[1].button("").clicked() {
                                Some(false)
                            } else {
                                None
                            }
                        })
                    })
                    .inner
                })
                .inner;
            match clicked {
                Some(true) => {
                    let name = name.take();
                    let id = *id;
                    project.add_label_modal = None;
                    if let std::collections::hash_map::Entry::Vacant(e) = project.labels.entry(id) {
                        e.insert(Label {
                                name,
                                color: COLORS[id as usize % COLORS.len()],
                            });
                    } else {
                        self.message_box = Some("This ID is taken already".to_string());
                        return;
                    }
                }
                Some(false) => project.add_label_modal = None,
                None => {}
            }
        }
        self.msg_box(ctx);
    }
}

impl App {
    pub fn open_project(&mut self, yaml_file_path: PathBuf) {
        let loaded_project = Project::load(yaml_file_path);
        match loaded_project {
            Ok(project) => {
                self.project = Some(project);
            }
            Err(err) => {
                self.message_box = Some(format!("{}", err));
            }
        }
    }

    pub fn label_segment(&mut self, image_rect: egui::Rect, label_id: u32, rect: egui::Rect) {
        let rect = rect.intersect(image_rect);
        // boohoo
        let project = self.project.as_mut().unwrap();
        let image = &mut project.images[project.image_index];
        let center = egui::Pos2 {
            x: (rect.center().x - image_rect.min.x) / image_rect.width(),
            y: (rect.center().y - image_rect.min.y) / image_rect.height(),
        };
        let size = egui::Vec2 {
            x: rect.width() / image_rect.width(),
            y: rect.height() / image_rect.height(),
        };
        image.segments.push(Segment {
            label_id,
            center,
            size,
        });

        if self.advance_on_accept {
            project.advance();
        }
    }

    pub fn read_inputs(&mut self, ctx: &egui::Context) -> Input {
        ctx.input(|r| {
            let key = |k| r.key_pressed(k);

            use egui::Key::*;
            Input {
                left: key(A),
                right: key(D),
                accept: key(Space),
                delete: key(X),
                clone: key(C),
                tool: if key(Q) {
                    Some(Tool::Stamp)
                } else if key(W) {
                    Some(Tool::Drag)
                } else if key(E) {
                    Some(Tool::Edit)
                } else {
                    None
                },
                hover_pos: r.pointer.hover_pos(),
                scroll_delta: if self.smooth_scroll {
                    r.smooth_scroll_delta
                } else {
                    r.raw_scroll_delta
                },
                // click: r.pointer.primary_clicked(),
            }
        })
    }

    pub fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("sidepanel")
            .resizable(false)
            .exact_width(256.0)
            .show(ctx, |ui| {
                self.label_buttons(ui);

                egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "saveload")
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        ui.vertical(|ui| {
                            help(ui);
                            self.save_load_buttons(ui);
                        });
                    });
            });
    }

    fn save_load_buttons(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |ui| {
            ui[0].vertical_centered_justified(|ui| {
                if ui
                    .add(egui::Button::new(""))
                    .on_hover_text("Load")
                    .clicked()
                {
                    let path = rfd::FileDialog::new()
                        .add_filter("yaml", &["yaml"])
                        .pick_file();

                    if let Some(path) = path {
                        self.open_project(path);
                    }
                }
            });
            ui[1].vertical_centered_justified(|ui| {
                let button = egui::Button::new("");
                if let Some(p) = &mut self.project {
                    if ui.add(button).on_hover_text("Save").clicked()
                        && let Err(err) = p.save()
                    {
                        self.message_box = Some(format!("{}", err));
                    }
                } else {
                    ui.scope(|ui| {
                        ui.disable();
                        ui.add(button);
                    });
                }
            });
        });
    }

    fn label_buttons(&mut self, ui: &mut egui::Ui) {
        if let Some(project) = &mut self.project {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Labels");
                egui_extras::TableBuilder::new(ui)
                    // .column(Column::auto())
                    .column(Column::remainder())
                    // .column(Column::auto())
                    // .column(Column::auto())
                    .body(|mut b| {
                        for (&i, label) in project.labels.iter() {
                            b.row(16.0, |mut row| {
                                // row.col(|ui| {
                                //     ui.checkbox(&mut false, ());
                                // });
                                row.col(|ui| {
                                    ui.style_mut().visuals.override_text_color = Some(label.color);
                                    if ui
                                        .add(
                                            egui::Button::new(&label.name)
                                                .selected(project.label_id == Some(i)),
                                        )
                                        .clicked()
                                    {
                                        project.label_id = Some(i);
                                    };
                                });
                                // row.col(|ui| {
                                //     ui.label("0");
                                // });
                                // row.col(|ui| {
                                //     ui.label("0");
                                // });
                            });
                        }
                        b.row(16.0, |mut row| {
                            row.col(|ui| {
                                if ui.add(egui::Button::new("+")).clicked() {
                                    project.add_label_modal =
                                        Some((1 + project.labels.len() as u32, String::new()));
                                };
                            });
                        });
                    });
                ui.style_mut().visuals.override_text_color = None;
                ui.separator();
                ui.heading("Tools");
                ui.columns(3, |ui| {
                    let mut tool = |i: usize, t, icon, hover| {
                        ui[i].vertical_centered_justified(|ui| {
                            if ui
                                .add(egui::Button::new(icon).selected(project.tool == t))
                                .on_hover_text(hover)
                                .clicked()
                            {
                                project.tool = t;
                            };
                        });
                    };
                    tool(0, Tool::Stamp, "", "Stamp tool");
                    tool(1, Tool::Drag, "", "Drag tool");
                    tool(2, Tool::Edit, "", "Edit tool");
                });
                ui.separator();
                ui.heading("Options");
                ui.horizontal(|ui| ui.checkbox(&mut self.smooth_scroll, "Smooth scroll"));
                ui.horizontal(|ui| ui.checkbox(&mut self.advance_on_accept, "Quick advance"));
                ui.separator();
                // ui.columns(3, |ui| {
                //     ui[0].vertical_centered_justified(|ui| {
                //         ui.add(egui::Button::new("Train").selected(false))
                //     });
                //     ui[1].vertical_centered_justified(|ui| {
                //         ui.add(egui::Button::new("Val").selected(false))
                //     });
                //     ui[2].vertical_centered_justified(|ui| {
                //         ui.add(egui::Button::new("Test").selected(false))
                //     });
                // });
                nav_buttons(ui, project);
            });
        }
    }

    pub fn central_panel(&mut self, ctx: &egui::Context, input: Input) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(project) = &mut self.project {
                if project.tool == Tool::Stamp {
                    let var_name = 0.25;

                    project.rect_size += egui::Vec2::splat(input.scroll_delta.y) * var_name;
                    project.rect_size.x += input.scroll_delta.x * var_name;
                }

                project.rect_size = project.rect_size.max(egui::Vec2 { x: 2.0, y: 2.0 });

                let image = &mut project.images[project.image_index];
                let res =
                    // ui .centered_and_justified(|ui| {
                        ui.add(egui::Image::new(&image.uri).sense(Sense::click_and_drag()));
                // })
                // .inner;
                let image_rect = res.rect;

                let mut highlighted_segment = None;
                for (i, segment) in image.segments.iter().enumerate() {
                    let rect = fun_name(image_rect, segment);
                    let label = &project.labels[&segment.label_id];
                    let color = label.color;
                    ui.painter().debug_rect(rect, color, &label.name);

                    if let Some(hover_pos) = input.hover_pos
                        && rect.contains(hover_pos)
                    {
                        highlighted_segment = Some((i, rect, segment.label_id, color));
                    }
                }

                if project.tool == Tool::Edit {
                    if let Some(drag) = &project.edit_drag
                        && res.dragged()
                    {
                        let delta = res.drag_delta();
                        let segment = &mut image.segments[drag.segment_index];
                        if drag.awesome == egui::Vec2::ZERO {
                            segment.center += delta / image_rect.size();
                        } else {
                            let shift = delta * drag.awesome.abs() / 2.0;
                            let resize = delta * drag.awesome;
                            segment.size += resize;
                            segment.center += shift;
                        }

                        ui.ctx().set_cursor_icon(drag.icon);
                    } else if res.drag_stopped() {
                        project.edit_drag = None;
                    } else if let Some(hover_pos) = input.hover_pos
                        && let Some(segment) = highlighted_segment
                    {
                        let (i, rect, _, _) = segment;
                        let pos = (hover_pos - rect.min) / rect.size() * 3.0;
                        let (x, y) = (pos.x as i32, pos.y as i32);
                        let icon = match (x, y) {
                            (0, 0) => egui::CursorIcon::ResizeNorthWest,
                            (1, 0) => egui::CursorIcon::ResizeNorth,
                            (2, 0) => egui::CursorIcon::ResizeNorthEast,
                            (0, 1) => egui::CursorIcon::ResizeWest,
                            (1, 1) => egui::CursorIcon::Move,
                            (2, 1) => egui::CursorIcon::ResizeEast,
                            (0, 2) => egui::CursorIcon::ResizeSouthWest,
                            (1, 2) => egui::CursorIcon::ResizeSouth,
                            (2, 2) => egui::CursorIcon::ResizeSouthEast,
                            _ => egui::CursorIcon::Default,
                        };
                        ui.ctx().set_cursor_icon(icon);
                        if res.drag_started() || res.is_pointer_button_down_on() {
                            project.edit_drag = Some(SegmentDrag {
                                segment_index: i,
                                awesome: egui::Vec2::new((x - 1) as f32, (y - 1) as f32)
                                    / image_rect.size(),
                                icon,
                            });
                        }
                    }
                }

                if let Some((i, rect, label, color)) = highlighted_segment {
                    ui.painter()
                        .rect_stroke(rect, 0.0, (3.0, color), egui::StrokeKind::Middle);
                    if input.delete {
                        image.segments.remove(i);
                    }
                    if input.clone {
                        project.rect_size = rect.size();
                        project.label_id = Some(label)
                    }
                }

                if let (Some(hover_pos), Some(label_id)) = (input.hover_pos, project.label_id) {
                    let label = &project.labels[&label_id];
                    if project.tool == Tool::Stamp {
                        let rect = egui::Rect::from_center_size(hover_pos, project.rect_size);
                        let rect = rect.intersect(image_rect);
                        ui.painter().debug_rect(rect, label.color, &label.name);
                        if input.accept || res.clicked() {
                            self.label_segment(image_rect, label_id, rect);
                        }
                    } else if project.tool == Tool::Drag {
                        if res.drag_started() {
                            project.drag_start_pos = Some(hover_pos);
                        }
                        let painter = ui.painter();
                        painter.hline(
                            Rangef::new(image_rect.left(), image_rect.right()),
                            hover_pos.y,
                            (1.0, label.color),
                        );
                        painter.vline(
                            hover_pos.x,
                            Rangef::new(image_rect.top(), image_rect.bottom()),
                            (1.0, label.color),
                        );
                        painter.circle_filled(hover_pos, 3.0, label.color);
                        painter.text(
                            hover_pos,
                            egui::Align2::LEFT_BOTTOM,
                            label.name.clone(),
                            FontId::monospace(12.0),
                            label.color,
                        );
                        if let Some(drag_start_pos) = project.drag_start_pos {
                            let rect = egui::Rect::from_two_pos(drag_start_pos, hover_pos);
                            painter.hline(Rangef::NOTHING, 100.0, (10.0, label.color));
                            painter.debug_rect(rect, label.color, &label.name);
                            painter.circle_filled(drag_start_pos, 3.0, label.color);
                            painter.circle_filled(hover_pos, 3.0, label.color);
                            if res.drag_stopped() {
                                project.drag_start_pos = None;
                                self.label_segment(image_rect, label_id, rect);
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn timeline(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            if let Some(project) = &mut self.project {
                let image_size = egui::Vec2::new(16.0, 9.0) * 4.0;
                let clip_rect = ui.clip_rect();
                let spacing = ui.spacing().item_spacing.x;
                // let offset = (image_size.x + spacing) * self.image_index as f32;
                let visible_images = (clip_rect.width() / (image_size.x + spacing)).ceil() as usize;
                let skip_images = project.image_index.saturating_sub(visible_images / 2);
                ui.horizontal(|ui| {
                    for (i, image) in project.images.iter().enumerate().skip(skip_images) {
                        if !clip_rect.contains(ui.next_widget_position()) {
                            break;
                        }

                        let res = ui.add(
                            egui::Image::new(&image.uri)
                                .fit_to_exact_size(image_size)
                                .sense(Sense::CLICK),
                        );

                        if res.clicked() {
                            project.image_index = i;
                        }

                        if project.image_index == i {
                            // let rect = res.rect.translate(egui::Vec2 { x: offset, y: 0.0 });
                            ui.painter().debug_rect(res.rect, Color32::RED, "");
                        }
                    }
                });
            }
        });
    }

    pub fn msg_box(&mut self, ctx: &egui::Context) {
        if let Some(msg) = &self.message_box {
            let clicked = egui::Modal::new("messagebox".into())
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label(msg);
                        ui.button("Ojejku").clicked()
                    })
                    .inner
                })
                .inner;
            if clicked {
                self.message_box = None;
            }
        };
    }
}

fn help(ui: &mut egui::Ui) {
    ui.label("Q - Stamp tool");
    ui.label("W - Drag tool");
    ui.label("E - Edit tool");
    ui.label("A - Previous image");
    ui.label("D - Next image");
    ui.label("X - Delete segment");
    ui.label("C - Clone segment");
    ui.label("Click space to stamp");
    ui.label("[Shift] Scroll to resize stamp");
}

fn nav_buttons(ui: &mut egui::Ui, project: &mut Project) {
    ui.columns(5, |ui| {
        ui[0].vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("")).clicked() {
                project.image_index = 0;
            }
        });
        ui[1].vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("")).clicked() {
                project.back();
            }
        });
        ui[2].add(
            egui::DragValue::new(&mut project.image_index).range(0..=(project.images.len() - 1)),
        );
        ui[3].vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("")).clicked() {
                project.advance();
            }
        });
        ui[4].vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("")).clicked() {
                project.image_index = project.images.len() - 1;
            }
        });
    });
}

pub fn fun_name(image_rect: egui::Rect, segment: &Segment) -> egui::Rect {
    egui::Rect::from_center_size(
        image_rect.min
            + egui::Vec2 {
                x: segment.center.x * image_rect.width(),
                y: segment.center.y * image_rect.height(),
            },
        egui::Vec2 {
            x: segment.size.x * image_rect.width(),
            y: segment.size.y * image_rect.height(),
        },
    )
}
