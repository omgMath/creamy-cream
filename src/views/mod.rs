//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//! The [`Layout`] component wraps all pages with a common header and footer.

mod home;
pub use home::Home;

mod ingredient;
pub use ingredient::Ingredients;

mod ingredient_new;
pub use ingredient_new::IngredientCreate;

mod layout;
pub use layout::Layout;
