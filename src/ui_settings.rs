use ahash::AHashMap as HashMap;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::general::*;
use crate::style::*;
use super::ui_settings_buttons::*;


// ===========================================================
// === SETUP SETTINGS LAYOUT ===

pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    // ===========================================================
    // === SETUP STYLES ===
    //# Here we declare all the styling that will be applied later. To use global settings, we import constants declaring project scale colors and font paths.

    let style_navigation = TextStyle {
        font: asset_server.load(GLOBAL_NAVIGATION_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_tab = TextStyle {
        font: asset_server.load(GLOBAL_TAB_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_category = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 40.0,
        color: SETTINGS_COLOR_CATEGORY,
    };
    let style_item = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };

    
    // ===========================================================
    // === SETUP WIDGETS AND ENTITIES ===
    //# This is where the layouting magic happens. Here we declare the positions and spawn relevant entities.

    //# Create SETTINGS in ROOT
    let settings = Widget::create(system, "settings", Box::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BACKGROUND in SETTINGS
    let background = Widget::create(system, &settings.end("background"), Box::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BACKGROUND
    let image = Widget::create(system, &background.end(""), Box::Solid {
        width: 3840,
        height: 2160,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, image.clone(), &ImageParams::default(), "settings/background.png");
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 3.0, y: 1.0 },
        relative_2: Vec2 { x: 15.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let button_return = Widget::create(system, &boundary.end("return"), Box::Solid {
        width: 3,
        height: 1,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    text_element_spawn!(commands, button_return, &TextParams::centerleft().styled(&style_navigation).scaled(35.0).with_height(80.0).at(10.0, 50.0), "RETURN",
        ColorHighlightEffect (style_navigation.color, GLOBAL_COLOR_HOVER),
        ReturnButton (),
        HoverEffectInput (),
        ColorHighlightEffectUpdater ()
    );

    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 3.0, y: 9.0 },
        relative_2: Vec2 { x: 90.0, y: 13.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let line = Widget::create(system, &boundary.end(""), Box::Solid {
        width: 3522,
        height: 4,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, line, &ImageParams::default(), "settings/line.png");


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BAR widget in SETTINGS
    let bar = Widget::create(system, &settings.end("bar"), Box::Relative {
        relative_1: Vec2 { x: 18.0, y: 1.0 },
        relative_2: Vec2 { x: 82.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BAR
    let boundary = Widget::create(system, &bar.end(""), Box::Solid {
        width: 28,
        height: 1,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();

    //# Generate grid of widgets in 'nameless'
    let names = textgrid![["tab 1"], ["tab 2"], ["tab 3"], ["tab 4"], ["tab 5"], ["tab 6"], ["tab 7"], ["tab 8"]];
    let grid = GridParams::new(&names).with_width(100.0).with_height(20.0).with_width_gap(10.0);
    grid_generate_inside(system, &boundary, &grid).unwrap();

    //# Loop over grid of widgets in 'nameless'
    for x in 0..names.len() {
        for y in 0..names[0].len() {

            //# Spawn image for widgets in 'nameless'
            let widget = Widget::new(&boundary.end(&names[x][y]));
            text_element_spawn!(commands, widget, &TextParams::center().styled(&style_tab).scaled(50.0).with_height(80.0), &names[x][y].to_uppercase(),
                ColorHighlightEffect (style_tab.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                ColorHighlightEffectUpdater ()
            );
        }
    }


    //# --------------------------------------------------------------------------------------------------------------
    
    //# Create 'nameless' widget in SETTINGS
    let boundary1 = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 10.0, y: 14.0 },
        relative_2: Vec2 { x: 90.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in 'nameless'
    let boundary2 = Widget::create(system, &boundary1.end(""), Box::Solid {
        width: 105,
        height: 100,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create DISPLAY widget in 'nameless'/'nameless' (skipping 2 nameless widgets at once)
    let display = Widget::create(system, &settings.add(&boundary1).add(&boundary2).end("display"), Box::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in DISPLAY
    let category = Widget::create(system, &display.end(""), Box::Solid {
        width: 1934,
        height: 96,
        vertical_anchor: -1.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, category.clone(), &ImageParams::default(), "settings/category.png");
    text_element_spawn!(commands, category.clone(), &TextParams::centerleft().styled(&style_category).scaled(40.0).at(2.0, 50.0), "Display");



    let names = textgrid![["Window mode", "Decorations", "Resizable window", "Title", "Resolution"]];
    let mut options = HashMap::new();
    options.insert("Window mode", textrow!["Windowed", "Borderless"]);
    options.insert("Resolution", textrow!["1920x1080", "1280x720", "720x720"]);


    let grid = GridParams::new(&names).with_width(96.0).with_height(11.0).with_width_gap_border(true).with_height_gap_border(true);
    let widget = grid_generate(system, &display.end("list"), Vec2::new(0.0, 16.0), &grid).unwrap();

    for x in 0..names.len() {
        for y in 0..names[0].len() {

            //# --------------------------------------------------------------------------------------------------------------

            //# Create OPTION widget in LIST
            let widget = Widget::new(&widget.end(&names[x][y]));
            text_element_spawn!(commands, widget.clone(), &TextParams::centerleft().styled(&style_item).scaled(40.0).at(2.0, 50.0), &names[x][y].to_uppercase(),
                ColorHighlightEffect (style_item.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                ColorHighlightEffectUpdater ()
            );

            //# Create 'nameless' widget in LIST
            let button = Widget::create(system, &widget.end(""), Box::Relative {
                relative_1: Vec2::new(47.0, 2.0),
                relative_2: Vec2::new(95.0, 98.0),
                ..Default::default()
            }.pack()).unwrap();

            //# Create BUTTON in 'nameless'
            let mut option = vec!["Enabled".to_string(), "Disabled".to_string()];
            if let Some (custom) = options.get(names[x][y].as_str()) {
                option = custom.to_vec();
            }

            let (button, component) = OptionButton::create(commands, asset_server, system, &button.end(&names[x][y]), Box::Relative::default().pack(), option);
            widget_spawn!(commands, button, component);

            //# --------------------------------------------------------------------------------------------------------------

        }
    }

}


// ===========================================================
// === INTERACTION SYSTEMS ===

#[derive(Component)]
pub struct HoverEffectInput ();
fn hover_effect_input(mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &HoverEffectInput)>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            let data_option = widget.fetch_mut(&mut system, "").unwrap().data_get_mut();
            match data_option {
                Option::Some ( data ) => {
                    data.f32s.insert("color_highlight_effect_slider".to_string() , 1.0);
                },
                Option::None => {
                    *data_option = Option::Some(Data::new());
                },
            }
        }
    }
}


#[derive(Component)]
struct ReturnButton ();
fn return_button_update (mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &ReturnButton)>, mouse_button_input: Res<Input<MouseButton>>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            if mouse_button_input.just_pressed(MouseButton::Left) {
                Widget::new("main_menu").fetch_mut(&mut system, "").unwrap().set_visibility(true);
                Widget::new("settings").fetch_mut(&mut system, "").unwrap().set_visibility(false);
            }

        }
    }
}



// ===========================================================
// === PACK ALL SYSTEMS TO PLUGIN ===

pub struct UISettingsPlugin;
impl Plugin for UISettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hover_effect_input)
            .add_systems(Update, return_button_update)
            .add_systems(Update, (option_button_text_update, option_button_update));
    }
}