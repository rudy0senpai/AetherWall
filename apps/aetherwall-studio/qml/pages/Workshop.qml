// Workshop view — Studio page
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
            Label { text: "Workshop"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
            SearchBar { placeholderText: "Search providers…"; }
        }
        Column {
            id: discovery
            anchors.fill: parent
            spacing: 12
            GridView {
                columns: 2
                delegate: Column {
                    radius: 8
                    color: "#14131B"
                    border.color: "#343044"
                    Label { text: "🔍 Anime Land"; font.pixelSize: 14; color: "#F5F3FA"; }
                    Label { text: "1,250 items"; font.pixelSize: 11; color: "#AAA5B8"; }
                    Button {
                        text: "Browse"; background: "#14131B"; border.color: "#343044"
                    }
                }
            }
        }
    }
}
