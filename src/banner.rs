
static banner: [&str] = [
    "This file was created automatically by {}. Modifications to this file will",
    "be lost when the next build is done.",
];

struct decoration {
    at_top:     &str,
    at_bottom:  &str,
    at_right:   &str,
    at_left:    &str,
    for_line:   &str,
};
