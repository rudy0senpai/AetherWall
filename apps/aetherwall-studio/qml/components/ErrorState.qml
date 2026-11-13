// ErrorState — error state screen
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property string title: "Something went wrong"
    property string message: "An unexpected error occurred. Please try again."
    property string actionText: "Retry"
    signal actionClicked()

    width: 300
    height: 180

    Column {
        anchors.centerIn: parent
        spacing: 12

        Text {
            text: "⚠️"
            font.pixelSize: 48
            color: "#FB7185"
        }

        Text {
            text: title
            font.pixelSize: 18
            font.weight: Font.Medium
            color: "#FB7185"
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
