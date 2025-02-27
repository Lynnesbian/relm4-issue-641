use relm4::prelude::*;
use relm4::gtk::prelude::*;

fn main() {
	let app = RelmApp::new("space.lynnesbian.relm4-template-issue-641");
	app.run::<Window>(());
}

struct Window(bool);

#[relm4::component(pub)]
impl SimpleComponent for Window {
	type Input = ();
	type Output = ();
	type Init = ();
	view! {
		#[root]
		gtk::Window {
			set_height_request: 240,
			set_width_request: 320,

			match model.0 {
				value => {
					gtk::Box {
						gtk::Label {
							set_label: &format!("This works: {:?}", model.0),
						},

						gtk::Label {
							#[watch]
							set_label: &format!("This also works: {:?}", value),
						},

						#[template]
						Inner {
							#[template_child]
							item {
								set_label: &format!("This template works: {:?}", model.0)
							}
						},

						#[template]
						Inner {
							#[template_child]
							item { // ERROR: cannot find value `item` in this scope
								#[watch]
								set_label: &format!("This doesn't: {:?}", value)
							}
						},

						#[template]
						Inner {
							#[template_child]
							item {
								set_label: &format!("This also doesn't: {:?}", value) // ERROR: cannot find value `value` in this scope
							}
						}
					}
				}
			},

			if let value = model.0 {
				#[template]
				Inner {
					#[template_child]
					item { // ERROR: cannot find value `item` in this scope
						#[watch] // removing this #[watch] attribute breaks the below line
						set_label: &format!("This if let binding also doesn't work: {:?}", value)
					}
				}
			} else {
				gtk::Box {}
			}
		}
	}

	fn init(_init: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
		let model = Self(true);
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for Inner {
	view! {
		#[name(item)]
		gtk::Label { }
	}
}
