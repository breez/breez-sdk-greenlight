package breez_sdk_notification

import android.annotation.SuppressLint
import android.content.Context
import android.content.pm.PackageManager
import android.content.pm.PackageManager.NameNotFoundException
import android.content.res.Resources.NotFoundException
import android.graphics.Color
import android.graphics.drawable.AdaptiveIconDrawable
import android.os.Build
import android.os.Bundle
import androidx.core.content.ContextCompat
import androidx.core.content.res.ResourcesCompat


class ResourceHelper {
    companion object {
        private const val ILLEGAL_RESOURCE_ID = 0

        private fun getBundle(context: Context): Bundle? {
            return try {
                context.packageManager.getApplicationInfo(
                    context.packageName, PackageManager.GET_META_DATA
                ).metaData
            } catch (_: NameNotFoundException) {
                null
            }
        }

        @SuppressLint("DiscouragedApi")
        private fun getResourceId(context: Context, name: String, defType: String): Int? {
            return context.resources.getIdentifier(name, defType, context.packageName)
                .takeIf { it != ILLEGAL_RESOURCE_ID }
        }

        private fun isDrawableValid(context: Context, resourceId: Int): Boolean {
            if (Build.VERSION.SDK_INT != Build.VERSION_CODES.O) {
                return true
            }

            return try {
                val icon = ResourcesCompat.getDrawable(context.resources, resourceId, context.theme)

                icon !is AdaptiveIconDrawable
            } catch (_: NotFoundException) {
                false
            }
        }

        fun getColor(context: Context, name: String, fallback: String): Int {
            val color =
                getResourceId(context, name, "color")?.let { ContextCompat.getColor(context, it) }
                    ?: run { getBundle(context)?.getInt(name, 0) }
            return color.takeUnless { it == 0 } ?: Color.parseColor(fallback)
        }

        fun getDrawable(context: Context, name: String, fallback: Int): Int {
            val id =
                getResourceId(context, name, "drawable")?.takeIf { isDrawableValid(context, it) }
                    ?: run {
                        getResourceId(context, name, "mipmap")?.takeIf {
                            isDrawableValid(
                                context,
                                it
                            )
                        }
                    }

            return id ?: fallback
        }

        fun getString(context: Context, name: String, fallback: String): String {
            return getString(context, name, null, fallback)
        }

        fun getString(
            context: Context, name: String, validateContains: String?, fallback: String
        ): String {
            val str = getResourceId(context, name, "string")?.let { context.getString(it) } ?: run {
                getBundle(context)?.getString(name, fallback) ?: run { fallback }
            }

            return if (validateContains == null || str.contains(validateContains)) str else fallback
        }
    }
}
