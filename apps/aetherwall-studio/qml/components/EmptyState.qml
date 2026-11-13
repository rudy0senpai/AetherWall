// EmptyState — empty state screen
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property string title: "No items"
    property string message: "Your library is empty. Import wallpapers to get started."
    property string actionText: "Import wallpapers"
    signal actionClicked()

    width: 300
    height: 180

    Column {
        anchors.centerIn: parent
        spacing: 12

        Text {
            text: "📂"
            font.pixelSize: 48
            color: "#7A728A"
        }

        Text {
            text: title
            font.pixelSize: 18
            font.weight: Font.Medium
            color: "#F5F3FA"
        }

        Text {
            text: message
            font.pixelSize: 13
            color: "#AAA5B8"
            anchors.horizontalCenter: parent.horizontalCenter
            wrapMode: Text.WordWrap
        }

        Button {
            text: actionText
            background: "#8B3DBB"
            color: "#F5F3FA"
            onClicked: actionClicked()
        }
    }
}
