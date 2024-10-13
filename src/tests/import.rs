use super::*;

make_wide_test!(import_items_one_line_wide, IMPORT_ITEMS_ONE_LINE);
make_test!(import_items_one_line_narrow, IMPORT_ITEMS_ONE_LINE);

make_wide_test!(import_items_multiline_wide, IMPORT_ITEMS);
make_test!(import_items_multiline_narrow, IMPORT_ITEMS);

const IMPORT_ITEMS_ONE_LINE: &str = r#"
#import touying: touying-slide-wrapper, components, utils, config-common, config-page, touying-slide
#import touying: touying-slide-wrapper, components, utils, config-common, config-page, touying-slide,
#import touying: (touying-slide-wrapper, components, utils, config-common, config-page, touying-slide)
#import touying: (touying-slide-wrapper, components, utils, config-common, config-page, touying-slide,)
#import touying: (touying-slide-wrapper, components, utils, config-common, config-page, touying-slide )
#import touying: (touying-slide-wrapper, components, utils, config-common, config-page, touying-slide, )
"#;

const IMPORT_ITEMS: &str = r#"
#import touying: (
  touying-slide-wrapper,
  components,
  utils,
  config-common,
  config-page,
  touying-slide,
)
#import touying: (
  touying-slide-wrapper,
  components,
  utils,
  config-common,
  config-page,
  touying-slide
)
#import touying: ( touying-slide-wrapper,
  components,
  utils,
  config-common,
  config-page,
  touying-slide,

)
#import touying: (touying-slide-wrapper,
  components,
  utils,
  config-common,
  config-page,
  touying-slide)
"#;
