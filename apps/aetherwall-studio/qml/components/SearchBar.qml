// SearchBar — search input field
import QtQuick
import QtQuick.Controls

TextField {
    id: search
    placeholderText: "Search…"
    background: "#14131B"
    border.color: "#343044"
    border.width: 1
    radius: 8
    padding: 8
    font.pixelSize: 13
    color: "#F5F3FA"

    Rectangle {
        id: clearButton
        width: 20
        height: 20
        radius: 10
        color: "#1D1A27"
        visible: visible && text.length > 0
        Label {
            text: "✕"
            anchors.centerIn: parent
            color: "#AAA5B8"
        }
        TapHandler {
            onClicked: {
                search.text = ""
                search.forceActiveFocus()
            }
        }
    }
}
