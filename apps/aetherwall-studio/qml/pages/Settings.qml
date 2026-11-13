// Settings view — Studio page
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Item {
    id: root
    width: parent.width
    height: parent.height
    Column {
        anchors.fill: parent
        anchors.margins: 16
        ToolBar {
            Label { text: "Settings"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
        }
        GroupBox {
            title: "Appearance"
            radius: 8
            color: "#14131B"
            border.color: "#343044"
            Column {
                spacing: 8
                Row {
                    Label { text: "Theme"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Button { text: "Dark"; background: "#1D1A27"; border.color: "#343044"; }
                }
                Row {
                    Label { text: "Accent"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Button { text: "Purple"; background: "#8B3DBB"; border.color: "#8B3DBB"; color: "#F5F3FA"; }
                }
                Row {
                    Label { text: "Font size"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Button { text: "Default"; background: "#14131B"; border.color: "#343044"; }
                }
            }
        }
        GroupBox {
            title: "Interface"
            radius: 8
            color: "#14131B"
            border.color: "#343044"
            Column {
                spacing: 8
                Row {
                    Label { text: "Compact mode"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Switch { checked: false; }
                }
            }
        }
        GroupBox {
            title: "Providers"
            radius: 8
            color: "#14131B"
            border.color: "#343044"
            Column {
                spacing: 8
                Row {
                    Label { text: "Anime pictures"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Switch { checked: true; }
                }
                Row {
                    Label { text: "Anime videos"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Switch { checked: false; }
                }
            }
        }
        GroupBox {
            title: "Storage"
            radius: 8
            color: "#14131B"
            border.color: "#343044"
            Column {
                spacing: 8
                Row {
                    Label { text: "Cache size"; font.pixelSize: 13; color: "#AAA5B8"; }
                    Label { text: "2.4 GB"; font.pixelSize: 13; color: "#F5F3FA"; }
                }
            }
        }
    }
}
