import xml.etree.ElementTree as ET
import re
import os
from pathlib import Path
import shutil

ICON_SRC_DIR="icons/"
ICON_DEST_DIR="src/icons"
ATTRIB_BLACK_LIST = ["data-loading"]
ROOT_ATTRIB_BLACK_LIST = ["width", "height", "viewBox"]
START_INDENT = 8

SVG_TEMPLATE="""\
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use dioxus::prelude::*;

//
// Do not edit. This file is generated automatically from
// the svg file.
//

#[derive(Props, PartialEq)]
pub struct IconProps<'a> {{
    #[props(default = {default_width})]
    pub width: u32,
    #[props(default = {default_height})]
    pub height: u32,
    #[props(default = "")]
    pub class: &'a str
}}

pub fn {icon_name}<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {{
    render! {{
{svg}
    }}
}}
"""

NODE_SVG_TEMPLATE="""\
        svg {{
            width: format_args!("{{}}", cx.props.width),
            height: format_args!("{{}}", cx.props.height),
            view_box: \"{viewbox}\",
            class: cx.props.class,
{children}
        }}\
"""


def to_snake_case(s):
    pattern = re.compile(r'(?<!^)(?=[A-Z])')
    return pattern.sub("_", s).lower()


def to_camel_case(s):
    return "".join([i.title() for i in s.split("_")])


def parse_tree(node: ET.Element, indent: int):
    output = ""
    indent_str = " " * indent
    indent_child = " " * (indent + 4)
    node_type = (node.tag.split("}")[1])
    output += "{}{} {{\n".format(indent_str, node_type)
    for attrib in node.attrib:
        if not attrib in ATTRIB_BLACK_LIST:
            attrib_name = to_snake_case(attrib.replace("-", "_"))
            attrib_value = node.attrib.get(attrib).replace("\"", "\\\"")
            output += "{}{}: \"{}\",\n".format(indent_child, attrib_name, attrib_value)
    for child in node:
        output += parse_tree(child, indent + 4)
    output += "{}}},\n".format(indent_str)
    return output


def parse_file(filepath):
    tree = ET.parse(filepath)
    root = tree.getroot()
    content = ""
    icon_name = to_camel_case(Path(filepath).stem)
    indent_child = " " * (START_INDENT + 4)
    default_width = int("".join(filter(str.isdigit, root.attrib.get("width", "50"))))
    default_height = int("".join(filter(str.isdigit, root.attrib.get("height", "50"))))
    viewbox = root.attrib.get("viewBox", "0 0 {} {}".format(default_width, default_height))
    for attrib in root.attrib:
        if not attrib in ROOT_ATTRIB_BLACK_LIST:
            attrib_name = to_snake_case(attrib.replace("-", "_"))
            attrib_value = root.attrib.get(attrib).replace("\"", "\\\"")
            content += "{}{}: \"{}\",\n".format(indent_child, attrib_name, attrib_value)
    for child in root:
        content += parse_tree(child, START_INDENT + 4)
    svg_node = NODE_SVG_TEMPLATE.format(viewbox=viewbox, children=content[:-1])
    svg = SVG_TEMPLATE.format(icon_name=icon_name, svg=svg_node, default_width=default_width, default_height=default_height)
    return (svg)


def convert_icons():
    if os.path.exists(ICON_DEST_DIR):
        shutil.rmtree(ICON_DEST_DIR, ignore_errors=True)
    os.mkdir(ICON_DEST_DIR)
    mod_rs_content = ""
    for file in os.listdir(ICON_SRC_DIR):
        print("Converting:", file)
        icon = parse_file(os.path.join(ICON_SRC_DIR, file))
        path = Path(os.path.join(ICON_DEST_DIR, file)).with_suffix(".rs")
        with open(path, "w+") as f:
            f.write(icon)
        basename = Path(file).stem
        mod_rs_content += "pub use {}::{};\n".format(basename, to_camel_case(basename))
        mod_rs_content += "mod {};\n".format(basename)
    with open(os.path.join(ICON_DEST_DIR, "mod.rs"), "w+") as f:
        f.write(mod_rs_content)


if __name__ == "__main__":
    convert_icons()
