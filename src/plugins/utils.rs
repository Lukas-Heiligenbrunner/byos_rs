pub fn html_body(content: String) -> String {
    let s = format!(
        "
    <!DOCTYPE html>
<html>
<head>
    <link rel='stylesheet' href='https://usetrmnl.com/css/latest/plugins.css'>
    <script src='https://usetrmnl.com/js/latest/plugins.js'></script>
</head>
<body class='environment trmnl'>
<div class='screen'>
{}
</div>
</body>
</html>
    ",
        content
    );

    s.to_string()
}
