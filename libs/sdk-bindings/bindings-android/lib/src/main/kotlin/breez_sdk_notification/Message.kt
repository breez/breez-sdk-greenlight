package breez_sdk_notification

import android.os.Parcel
import android.os.Parcelable

data class Message(val priority: Int, val type: String?, val payload: String?): Parcelable {
    constructor(parcel: Parcel) : this(parcel.readInt(), parcel.readString(), parcel.readString())

    override fun describeContents(): Int {
        return 0
    }

    override fun writeToParcel(parcel: Parcel, flags: Int) {
        parcel.writeInt(priority)
        parcel.writeString(type)
        parcel.writeString(payload)
    }

    companion object CREATOR : Parcelable.Creator<Message> {
        val PRIORITY_UNKNOWN = 0
        val PRIORITY_HIGH = 1
        val PRIORITY_NORMAL = 2

        override fun createFromParcel(parcel: Parcel): Message {
            return Message(parcel)
        }

        override fun newArray(size: Int): Array<Message?> {
            return arrayOfNulls(size)
        }
    }
}
