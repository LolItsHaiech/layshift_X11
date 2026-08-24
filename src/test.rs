#[cfg(test)]
mod layout_tests {
    use crate::layout::{KeyFamily, Layout};

    fn set_up() -> Layout {
        Layout::new("en:qwerty").unwrap()
    }

    #[test]
    fn test_get_normal_character_index() {
        let layout = set_up();

        assert_eq!(
            layout.get_character_index('`'),
            Some((KeyFamily::Normal, 0))
        );
        assert_eq!(
            layout.get_character_index('f'),
            Some((KeyFamily::Normal, 29))
        );
        assert_eq!(
            layout.get_character_index(' '),
            Some((KeyFamily::Normal, 47))
        );
    }

    #[test]
    fn test_get_shift_character_index() {
        let layout = set_up();

        assert_eq!(layout.get_character_index('~'), Some((KeyFamily::Shift, 0)));
        assert_eq!(
            layout.get_character_index('A'),
            Some((KeyFamily::Shift, 26))
        );
        assert_eq!(
            layout.get_character_index('?'),
            Some((KeyFamily::Shift, 46))
        );
    }

    #[test]
    fn test_get_unknown_character_index() {
        let layout = set_up();

        assert_eq!(layout.get_character_index('ب'), None);
        assert_eq!(layout.get_character_index('﷼'), None);
        assert_eq!(layout.get_character_index('随'), None);
    }

    #[test]
    fn test_get_normal_index_character() {
        let layout = set_up();

        assert_eq!(layout.get_index_character(KeyFamily::Normal, 1), Some('1'));
        assert_eq!(layout.get_index_character(KeyFamily::Normal, 30), Some('g'));
        assert_eq!(layout.get_index_character(KeyFamily::Normal, 46), Some('/'));
    }

    #[test]
    fn test_get_shift_index_character() {
        let layout = set_up();

        assert_eq!(layout.get_index_character(KeyFamily::Shift, 3), Some('#'));
        assert_eq!(layout.get_index_character(KeyFamily::Shift, 47), Some(' '));
        assert_eq!(layout.get_index_character(KeyFamily::Shift, 36), Some('"'));
    }
}

#[cfg(test)]
mod mapper_tests {
    use crate::layout::{Layout, map_string};

    fn set_up() -> (Layout, Layout) {
        (
            Layout::new("en:qwerty").unwrap(),
            Layout::new("fa:winkey").unwrap(),
        )
    }

    #[test]
    fn test_map_same_layouts() {
        let (source_layout, _) = set_up();

        assert_eq!(
            map_string("hello world.", &source_layout, &source_layout),
            "hello world."
        ); // only normal characters
        assert_eq!(
            map_string("HELLO WORLD!", &source_layout, &source_layout),
            "HELLO WORLD!"
        ); // only shift characters
        assert_eq!(
            map_string("Hello from Earth...", &source_layout, &source_layout),
            "Hello from Earth..."
        ); // normal and shift characters
        assert_eq!(
            map_string("Hello from Earth🌏!", &source_layout, &source_layout),
            "Hello from Earth🌏!"
        ); // normal, shift and unknown characters
    }

    #[test]
    fn test_map_only_normal_characters() {
        let (layout1, layout2) = set_up();

        assert_eq!(map_string("sghl nkdh.", &layout1, &layout2), "سلام دنیا.");
        assert_eq!(
            map_string("اثممخ صخقمی.", &layout2, &layout1),
            "hello world."
        );
    }

    #[test]
    fn test_map_only_shift_characters() {
        let (layout1, layout2) = set_up();

        assert_eq!(map_string("SGHL N,KDH!", &layout1, &layout2), "ُۀآ» إئ«ِآ!");
        assert_eq!(map_string("آٍ»»[ ٌ[﷼»ِ!", &layout2, &layout1), "HELLO WORLD!");
    }

    #[test]
    fn test_map_only_unknown_characters() {
        let (layout1, layout2) = set_up();

        assert_eq!(
            map_string("Hello world!", &layout2, &layout1),
            "Hello world!"
        );
        assert_eq!(map_string("سلام دنیا!", &layout1, &layout2), "سلام دنیا!");
    }

    #[test]
    fn test_map_all_types_of_characters() {
        let (layout1, layout2) = set_up();

        assert_eq!(
            map_string("sghl nkdh🌏!", &layout1, &layout2),
            "سلام دنیا🌏!"
        );
        assert_eq!(
            map_string("آثممخ صخقمی🌏!", &layout2, &layout1),
            "Hello world🌏!"
        );
    }
}
