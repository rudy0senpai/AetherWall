// Diagnostics view — Studio page
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
            Label { text: "Diagnostics"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
            Button { text: "Clear log"; background: "#14131B"; border.color: "#343044"; }
            Button { text: "Export"; background: "#14131B"; border.color: "#343044"; }
        }
        Column {
            id: logView
            width: parent.width
            height: 200
            radius: 8
            color: "#14131B"
            border.color: "#343044"
            Text {
                text: "[INFO] 13:45:22 — Studio connected to daemon\n[INFO] 13:45:22 — Window: 1440×900\n[INFO] 13:45:23 — Display topology: 2 displays\n[INFO] 13:46:01 — Library scan: 119 items\n[WARN] 13:47:15 — Web runtime crashed, recovered\n[INFO] 13:47:15 — Runtime restarted\n[INFO] 13:48:00 — Daemon heartbeat: OK"
                font.pixelSize: 9
                color: "#AAA5B8"
                wrapMode: Text.WordWrap
                anchors.fill: parent
                anchors.margins: 8
            }
        }
        Column {
            spacing: 8
            Card {
                radius: 8
                color: "#14131B"
                border.color: "#343044"
                Label { text: "Image runtime"; font.pixelSize: 13; color: "#F5F3FA"; anchors.left: parent.left; anchors.top: parent.top; anchors.margins: 8; }
                ProgressBar { value: 1.0; background: "#1D1A27"; border.color: "#343044"; } // running
            }
            Card {
                radius: 8
                color: "#14131B"
                border.color: "#343044"
                Label { text: "Qt WebEngine"; font.pixelSize: 13; color: "#F5F3FA"; anchors.left: parent.left; anchors.top: parent.top; anchors.margins: 8; }
                ProgressBar { value: 0; background: "#1D1A27"; border.color: "#343044"; }
            }
            Card {
                radius: 8
                color: "#14131B"
                border.color: "#343044"
                Label { text: "Game HTML5"; font.pixelSize: 13; color: "#F5F3FA"; anchors.left: parent.left; anchors.top: parent.top; anchors.margins: 8; }
                ProgressBar { value: 0; background: "#1D1A27"; border.color: "#343044"; }
            }
        }
    }
}
