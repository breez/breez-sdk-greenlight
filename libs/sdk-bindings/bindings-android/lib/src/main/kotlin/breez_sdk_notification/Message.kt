package breez_sdk_notification

import android.content.Intent
import android.os.Build
import android.os.Parcel
import android.os.Parcelable

data class Message(val type: String?, val payload: String?) : Parcelable {
    constructor(parcel: Parcel) : this(parcel.readString(), parcel.readString())

    override fun describeContents(): Int {
        return 0
    }

    override fun writeToParcel(parcel: Parcel, flags: Int) {
        parcel.writeString(type)
        parcel.writeString(payload)
    }

    companion object CREATOR : Parcelable.Creator<Message> {
        @Suppress("DEPRECATION")
        fun createFromIntent(intent: Intent?): Message? {
            return intent?.let {
                return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) it.getParcelableExtra(
                    Constants.EXTRA_REMOTE_MESSAGE,
                    Message::class.java
                ) else it.getParcelableExtra(Constants.EXTRA_REMOTE_MESSAGE)
            }
        }

        override fun createFromParcel(parcel: Parcel): Message {
            return Message(parcel)
        }

        override fun newArray(size: Int): Array<Message?> {
            return arrayOfNulls(size)
        }
    }
}
