macro_rules! make_macro {
    (
        $dollar: tt,
        $name: ident,
        $input: ident,
        $(use { $($other: ident),+ },)?
        $init: block
    ) => {
        #[macro_export]
        macro_rules! $name {
            (@common $dollar $input: ident) => {{
                use $dollar crate::gemtext::RawLine;
                $($(use $dollar crate::gemtext::$other;)+)?
                let $input = $dollar $input;
                $init
            }};
            ($dollar input: literal) => {{
                let $input = ::std::borrow::Cow::Borrowed($dollar input);
                $dollar crate::$name!(@common $input)
            }};
            ($dollar base: literal, $dollar($dollar arg: tt)+) => {
                let $input = ::std::borrow::Cow::Owned(::std::format!($dollar base, $dollar ($dollar arg)+));
                $dollar crate::$name!(@common $input)
            }
        }
    }
}

make_macro!($, blockquote, input, { RawLine::Blockquote(input) });
make_macro!($, h1, input, use { Heading }, { RawLine::Heading(Heading::H1(input)) });
make_macro!($, h2, input, use { Heading }, { RawLine::Heading(Heading::H2(input)) });
make_macro!($, h3, input, use { Heading }, { RawLine::Heading(Heading::H3(input)) });
make_macro!($, list_item, input, { RawLine::ListItem(input) });
make_macro!($, text, input, { RawLine::Text(input) });

#[macro_export]
macro_rules! link {
    (@common $url: expr, $text: expr) => {{
        let target = $url;
        $crate::gemtext::RawLine::Link($crate::gemtext::Link::new(
            target,
            $text,
        ).expect("URL provided to link!() was invalid"))
    }};
    (@fmt $base: literal) => {
        ::std::borrow::Cow::Borrowed($base)
    };
    (@fmt $base: literal, $($arg: tt),+) => {
        ::std::borrow::Cow::Owned(::std::format!($base, $($arg),+))
    };
    ($url: literal $(, $url_arg: tt)*) => {
        $crate::link!(@common
            $crate::link!(@fmt $url $(, $url_arg)*),
            None
        )
    };
    ($url: literal $(, $url_arg: tt)* -> $text: literal $(, $text_arg: tt)*) => {
        $crate::link!(@common
            $crate::link!(@fmt $url $(, $url_arg)*),
            Some($crate::link!(@fmt $text $(, $text_arg)*))
        )
    };
}

#[macro_export]
macro_rules! preformatted {
    (@common $alt_text: expr, $text: expr) => {
        $crate::gemtext::RawLine::Preformatted($crate::gemtext::Preformatted {
            alt_text: $alt_text,
            text: $text,
        })
    };
    (@fmt $base: literal) => {
        ::std::borrow::Cow::Borrowed($base)
    };
    (@fmt $base: literal, $($arg: tt),+) => {
        ::std::borrow::Cow::Owned(::std::format!($url, $($arg),+))
    };
    ($text: literal $(, $text_arg: tt)*) => {
        $crate::preformatted!(@common
            None,
            $crate::preformatted!(@fmt $url $(, $url_arg)*)
        )
    };
    ($alt: literal $(, $alt_arg: tt)*: $text: literal $(, $text_arg: tt)*) => {
        $crate::preformatted!(@common
            Some($crate::preformatted!(@fmt $alt $(, $alt_arg)*)),
            $crate::preformatted!(@fmt $text $(, $text_arg)*)
        )
    };
}

