use gtk::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkApplicationExt, IsA, WidgetExt};

use crate::component::Component;
use crate::component::ComponentController;
use crate::Application;
use crate::ComponentBuilder;

/// An app that runs the main application.
#[derive(Debug)]
pub struct RelmApp<C: Component> {
    bridge: ComponentBuilder<C>,

    /// The application that's used internally to setup
    /// and run the application.
    ///
    /// Depending on your feature flag this is either
    /// [`gtk::Application`] or [`adw::Application`].
    pub app: Application,
}

impl<C: Component> RelmApp<C>
where
    C::Root: IsA<gtk::Window> + WidgetExt,
{
    /// Create a Relm4 application.
    pub fn new(app_id: &str) -> Self {
        let app = Application::builder().application_id(app_id).build();

        Self::with_app(app)
    }

    /// Create a Relm4 application.
    pub fn with_app(app: impl IsA<Application> + Cast) -> Self {
        gtk::init().unwrap();

        #[cfg(feature = "libadwaita")]
        adw::init();

        let bridge = ComponentBuilder::<C>::new();

        Self {
            bridge,
            app: app.upcast(),
        }
    }

    /// Runs the application, returns once the application is closed.
    ///
    /// Unlike [`gtk::prelude::ApplicationExtManual::run`], this function
    /// does not handle command-line arguments. To pass arguments to GTK, use
    /// [`RelmApp::run_with_args`].
    pub fn run(self, payload: C::InitParams) {
        self.run_with_args::<&str>(payload, &[]);
    }

    /// Runs the application with the provided command-line arguments, returns once the application
    /// is closed.
    pub fn run_with_args<S>(self, payload: C::InitParams, args: &[S])
    where
        S: AsRef<str>,
    {
        let RelmApp { bridge, app } = self;
        let controller = bridge.launch(payload).detach();
        let window = controller.widget().clone();

        app.connect_activate(move |app| {
            app.add_window(window.as_ref());
            window.show();
        });

        app.run_with_args(args);
    }
}
