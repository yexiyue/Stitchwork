package com.yexiyue.stitchwork

import android.app.Activity
import android.util.Log
import com.azhon.appupdate.manager.DownloadManager
import com.toolsetlink.upgradelink.api.Client
import com.toolsetlink.upgradelink.api.models.ApkUpgradeRequest
import com.toolsetlink.upgradelink.api.models.ApkUpgradeResponse
import com.toolsetlink.upgradelink.api.models.Config

object UpdateHelper {
    private const val TAG = "AppUpdate"

    fun checkUpdate(activity: Activity) {
        val appKey = BuildConfig.UPGRADE_APP_KEY
        val accessKey = BuildConfig.UPGRADE_ACCESS_KEY
        val secretKey = BuildConfig.UPGRADE_SECRET_KEY

        if (appKey.isBlank() || accessKey.isBlank() || secretKey.isBlank()) {
            Log.w(TAG, "UpgradeLink credentials not configured, skipping update check")
            return
        }

        val config = Config().apply {
            this.accessKey = accessKey
            this.secretKey = secretKey
        }
        val client = Client(config)

        val request = ApkUpgradeRequest(
            appKey,
            BuildConfig.VERSION_CODE,
            0, // 0 = get latest version
            android.os.Build.MODEL,
            android.os.Build.DEVICE,
            0  // patchAlgo: 0 = no incremental update
        )

        client.ApkUpgradeAsync(request, object : Client.Callback<ApkUpgradeResponse> {
            override fun onSuccess(result: ApkUpgradeResponse) {
                Log.d(TAG, "Upgrade check response: $result")

                val data = result.data ?: return
                if (data.urlPath.isNullOrBlank()) {
                    Log.d(TAG, "No update available")
                    return
                }
                if (data.versionCode == null || data.versionCode <= BuildConfig.VERSION_CODE) {
                    Log.d(TAG, "Already on latest version")
                    return
                }

                activity.runOnUiThread {
                    startDownload(activity, data)
                }
            }

            override fun onFailure(e: Throwable) {
                Log.e(TAG, "Update check failed: ${e.message}")
            }
        })
    }

    private fun startDownload(
        activity: Activity,
        data: com.toolsetlink.upgradelink.api.models.ApkUpgradeDataResponse
    ) {
        // upgradeType: 1 = optional, 2 = forced
        val forced = data.upgradeType == 2

        val builder = DownloadManager.Builder(activity).run {
            apkUrl(data.urlPath)
            apkName("stitchwork_update.apk")
            smallIcon(R.mipmap.ic_launcher)
            apkVersionCode(data.versionCode)
            apkVersionName(data.versionName ?: "")
            apkDescription(data.promptUpgradeContent ?: "")
            forcedUpgrade(forced)
            showNotification(true)
            jumpInstallPage(true)
            // MD5 checksum for download verification
            if (!data.urlFileMd5.isNullOrBlank()) {
                apkMD5(data.urlFileMd5)
            }
            build()
        }
        builder?.download()
    }
}
