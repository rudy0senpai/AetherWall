// Downloads view — Studio page
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
            Label { text: "Downloads"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
            Button { text: "Refresh"; background: "#14131B"; border.color: "#343044"; }
        }
        Item {
            height: 160
            Label { text: "⬇️ No downloads yet"; font.pixelSize: 18; color: "#AAA5B8"; anchors.centerIn: parent; }
            Label { text: "Import or discover wallpapers and they will appear here."; font.pixelSize: 14; color: "#7A728A"; anchors.topMargin: 8; anchors.horizontalCenter: parent.horizontalCenter; }
            Button { text: "Browse downloads"; background: "#8B3DBB"; color: "#F5F3FA"; anchors.topMargin: 16; anchors.horizontalCenter: parent.horizontalCenter; }
        }
    }
}
