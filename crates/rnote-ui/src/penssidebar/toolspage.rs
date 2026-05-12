// Imports
use crate::RnAppWindow;
use gtk4::{
    Button, CompositeTemplate, MenuButton, Popover, SpinButton, ToggleButton, Widget, glib,
    glib::clone, prelude::*, subclass::prelude::*,
};
use rnote_engine::document::Layout;
use rnote_engine::pens::pensconfig::toolsconfig::ToolStyle;

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/penssidebar/toolspage.ui")]
    pub(crate) struct RnToolsPage {
        #[template_child]
        pub(crate) toolstyle_verticalspace_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) toolstyle_offsetcamera_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) toolstyle_zoom_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) toolstyle_laser_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) verticalspace_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub(crate) verticalspace_popover: TemplateChild<Popover>,
        #[template_child]
        pub(crate) verticalspace_popover_close_button: TemplateChild<Button>,
        #[template_child]
        pub(crate) verticalspace_limit_movement_vertical_bordersrow: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(crate) verticalspace_limit_movement_horizontal_bordersrow:
            TemplateChild<adw::SwitchRow>,

        #[template_child]
        pub(crate) goto_page_separator: TemplateChild<gtk4::Separator>,
        #[template_child]
        pub(crate) goto_page_spinbutton: TemplateChild<SpinButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RnToolsPage {
        const NAME: &'static str = "RnToolsPage";
        type Type = super::RnToolsPage;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RnToolsPage {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            self.dispose_template();
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for RnToolsPage {}
}

glib::wrapper! {
    pub(crate) struct RnToolsPage(ObjectSubclass<imp::RnToolsPage>)
        @extends Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl Default for RnToolsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl RnToolsPage {
    pub(crate) fn new() -> Self {
        glib::Object::new()
    }

    #[allow(unused)]
    pub(crate) fn tool_style(&self) -> Option<ToolStyle> {
        let imp = self.imp();

        if imp.toolstyle_verticalspace_toggle.is_active() {
            Some(ToolStyle::VerticalSpace)
        } else if imp.toolstyle_offsetcamera_toggle.is_active() {
            Some(ToolStyle::OffsetCamera)
        } else if imp.toolstyle_zoom_toggle.is_active() {
            Some(ToolStyle::Zoom)
        } else if imp.toolstyle_laser_toggle.is_active() {
            Some(ToolStyle::Laser)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub(crate) fn verticalspace_menubutton(&self) -> MenuButton {
        self.imp().verticalspace_menubutton.get()
    }

    #[allow(unused)]
    pub(crate) fn set_tool_style(&self, style: ToolStyle) {
        let imp = self.imp();

        match style {
            ToolStyle::VerticalSpace => imp.toolstyle_verticalspace_toggle.set_active(true),
            ToolStyle::OffsetCamera => imp.toolstyle_offsetcamera_toggle.set_active(true),
            ToolStyle::Zoom => imp.toolstyle_zoom_toggle.set_active(true),
            ToolStyle::Laser => imp.toolstyle_laser_toggle.set_active(true),
        }
    }

    pub(crate) fn init(&self, appwindow: &RnAppWindow) {
        let imp = self.imp();
        let verticalspace_popover = imp.verticalspace_popover.get();

        imp.toolstyle_verticalspace_toggle.connect_toggled(clone!(
            #[weak]
            appwindow,
            move |toggle| {
                if !toggle.is_active() {
                    return;
                }
                appwindow
                    .engine_config()
                    .write()
                    .pens_config
                    .tools_config
                    .style = ToolStyle::VerticalSpace;

                if let Some(canvas) = appwindow.active_tab_canvas() {
                    let widget_flags = canvas.engine_mut().reinstall_pen_current_style();
                    canvas.emit_handle_widget_flags(widget_flags);
                };
            }
        ));

        imp.toolstyle_offsetcamera_toggle.connect_toggled(clone!(
            #[weak]
            appwindow,
            move |toggle| {
                if !toggle.is_active() {
                    return;
                }
                appwindow
                    .engine_config()
                    .write()
                    .pens_config
                    .tools_config
                    .style = ToolStyle::OffsetCamera;

                if let Some(canvas) = appwindow.active_tab_canvas() {
                    let widget_flags = canvas.engine_mut().reinstall_pen_current_style();
                    canvas.emit_handle_widget_flags(widget_flags);
                };
            }
        ));

        imp.toolstyle_zoom_toggle.connect_toggled(clone!(
            #[weak]
            appwindow,
            move |toggle| {
                if !toggle.is_active() {
                    return;
                }
                appwindow
                    .engine_config()
                    .write()
                    .pens_config
                    .tools_config
                    .style = ToolStyle::Zoom;

                if let Some(canvas) = appwindow.active_tab_canvas() {
                    let widget_flags = canvas.engine_mut().reinstall_pen_current_style();
                    canvas.emit_handle_widget_flags(widget_flags);
                };
            }
        ));

        imp.toolstyle_laser_toggle.connect_toggled(clone!(
            #[weak]
            appwindow,
            move |toggle| {
                if !toggle.is_active() {
                    return;
                }
                appwindow
                    .engine_config()
                    .write()
                    .pens_config
                    .tools_config
                    .style = ToolStyle::Laser;

                if let Some(canvas) = appwindow.active_tab_canvas() {
                    let widget_flags = canvas.engine_mut().reinstall_pen_current_style();
                    canvas.emit_handle_widget_flags(widget_flags);
                };
            }
        ));

        imp.verticalspace_menubutton.connect_active_notify(clone!(
            #[weak(rename_to = toolspage)]
            self,
            move |menubutton| {
                if menubutton.is_active() {
                    toolspage.set_tool_style(ToolStyle::VerticalSpace);
                }
            }
        ));

        imp.verticalspace_popover_close_button
            .connect_clicked(clone!(
                #[weak]
                verticalspace_popover,
                move |_| {
                    verticalspace_popover.popdown();
                }
            ));

        imp.verticalspace_limit_movement_vertical_bordersrow
            .get()
            .connect_active_notify(clone!(
                #[weak]
                appwindow,
                move |row| {
                    appwindow
                        .engine_config()
                        .write()
                        .pens_config
                        .tools_config
                        .verticalspace_tool_config
                        .limit_movement_vertical_borders = row.is_active();
                }
            ));

        imp.verticalspace_limit_movement_horizontal_bordersrow
            .get()
            .connect_active_notify(clone!(
                #[weak]
                appwindow,
                move |row| {
                    appwindow
                        .engine_config()
                        .write()
                        .pens_config
                        .tools_config
                        .verticalspace_tool_config
                        .limit_movement_horizontal_borders = row.is_active();
                }
            ));

        // Go-to-page spinbutton — same wiring as the font-size spinbutton in typewriterpage.
        // Range and visibility are set in refresh_ui(); here we only wire the signal.
        imp.goto_page_spinbutton.set_increments(1.0, 5.0);
        imp.goto_page_spinbutton.set_range(1.0, 1.0);
        imp.goto_page_spinbutton.set_value(1.0);

        imp.goto_page_spinbutton.connect_activate(clone!(
            #[weak]
            appwindow,
            #[weak(rename_to = toolspage)]
            self,
            move |_| {
                toolspage.jump_to_page(&appwindow);
            }
        ));
    }

    fn jump_to_page(&self, appwindow: &RnAppWindow) {
        let Some(canvas) = appwindow.active_tab_canvas() else {
            return;
        };

        let target_page = (self.imp().goto_page_spinbutton.value() as u32).max(1);

        let (offset_x, offset_y) = {
            let engine = canvas.engine_ref();

            if !matches!(
                engine.document.config.layout,
                Layout::FixedSize | Layout::ContinuousVertical
            ) {
                return;
            }

            let zoom = engine.camera.zoom();
            let format_height = engine.document.config.format.height();
            let format_width = engine.document.config.format.width();
            let shadow = rnote_engine::Document::SHADOW_WIDTH;

            let y = (target_page.saturating_sub(1)) as f64 * format_height * zoom
                - shadow * zoom;

            let parent_w = canvas.parent().map(|p| p.width() as f64).unwrap_or(0.0);
            let x = if format_width * zoom <= parent_w {
                (format_width * 0.5 * zoom) - parent_w * 0.5
            } else {
                -shadow * zoom
            };

            (x, y)
        };

        let widget_flags = canvas
            .engine_mut()
            .camera_set_offset_expand(na::vector![offset_x, offset_y]);
        appwindow.handle_widget_flags(widget_flags, &canvas);
    }

    pub(crate) fn refresh_ui(&self, appwindow: &RnAppWindow) {
        let tools_config = appwindow
            .engine_config()
            .read()
            .pens_config
            .tools_config
            .clone();

        self.set_tool_style(tools_config.style);

        let imp = self.imp();
        imp.verticalspace_limit_movement_vertical_bordersrow
            .set_active(
                tools_config
                    .verticalspace_tool_config
                    .limit_movement_horizontal_borders,
            );
        imp.verticalspace_limit_movement_horizontal_bordersrow
            .set_active(
                tools_config
                    .verticalspace_tool_config
                    .limit_movement_vertical_borders,
            );

        // Show/hide goto-page spinbutton depending on layout
        if let Some(canvas) = appwindow.active_tab_canvas() {
            let engine = canvas.engine_ref();
            let layout = engine.document.config.layout;
            let is_paged = matches!(layout, Layout::FixedSize | Layout::ContinuousVertical);

            imp.goto_page_separator.set_visible(is_paged);
            imp.goto_page_spinbutton.set_visible(is_paged);

            if is_paged {
                let format_height = engine.document.config.format.height();
                let n_pages = if format_height > 0.0 {
                    (engine.document.height / format_height).ceil()
                } else {
                    1.0
                }
                .max(1.0);

                // Update range, clamping the current value if needed
                let current = imp.goto_page_spinbutton.value().clamp(1.0, n_pages);
                imp.goto_page_spinbutton.set_range(1.0, n_pages);
                imp.goto_page_spinbutton.set_value(current);
            }
        } else {
            imp.goto_page_separator.set_visible(false);
            imp.goto_page_spinbutton.set_visible(false);
        }
    }
}
