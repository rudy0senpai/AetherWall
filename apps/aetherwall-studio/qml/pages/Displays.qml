// Displays view — Studio page
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
            Label { text: "Displays"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
            Button { text: "Refresh"; background: "#14131B"; border.color: "#343044"; }
            Button { text: "+ Add display"; background: "#14131B"; border.color: "#343044"; }
        }
        Column {
            id: displayList
            anchors.fill: parent
            spacing: 8
            Repeater {
                model: DisplaysModel.displayList
                Card {
                    radius: 8
                    color: "#14131B"
                    border.color: "#343044"
                    Label { text: "Primary Display (1920×1080)"; font.pixelSize: 14; color: "#F5F3FA"; anchors.left: parent.left; anchors.top: parent.top; anchors.margins: 8; }
                    Label { text: "HDMI 1 · scale 100% · 60 Hz"; font.pixelSize: 11; color: "#AAA5B8"; anchors.topMargin: 4; anchors.left: parent.left; }
                    Row {
                        spacing: 8
                        Label { text: "Primary"; background: "#1D1A27"; border.color: "#343044"; color: "#34D399"; padding: 4; border-radius: 4; font.pixelSize: 11; }
                        Button { text: "Set as primary"; background: "#14131B"; border.color: "#343044"; }
                    }
                }
            }
        }
    }
}
