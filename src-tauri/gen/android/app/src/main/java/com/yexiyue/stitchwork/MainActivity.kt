package com.yexiyue.stitchwork

import android.Manifest
import android.app.AlertDialog
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.azhon.appupdate.config.UpdateConfiguration
import com.azhon.appupdate.listener.OnDownloadListener
import com.azhon.appupdate.manager.DownloadManager
import com.toolsetlink.upgradelink.api.Client
import com.toolsetlink.upgradelink.api.models.UrlUpgradeRequest
import com.toolsetlink.upgradelink.api.models.UrlUpgradeResponse
import java.io.File

class MainActivity : TauriActivity() {

    companion object {
        private const val PERMISSION_REQUEST_CODE = 1001
        private const val INSTALL_PACKAGES_REQUEST_CODE = 1002

        // UpgradeLink 配置 - 请替换为你的实际配置
        private const val ACCESS_KEY_ID = "你的 accessKeyId"
        private const val ACCESS_KEY_SECRET = "你的 accessKeySecret"
        private const val APP_UNIQUE_ID = "你的应用唯一标识"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        // 检查并请求必要权限
        checkAndRequestPermissions()

        // 检查更新
        checkForUpdate()
    }

    /**
     * 检查并请求必要权限
     */
    private fun checkAndRequestPermissions() {
        val permissions = mutableListOf<String>()

        // Android 10 及以下需要存储权限
        if (Build.VERSION.SDK_INT <= Build.VERSION_CODES.P) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE)
                != PackageManager.PERMISSION_GRANTED
            ) {
                permissions.add(Manifest.permission.WRITE_EXTERNAL_STORAGE)
            }
        }

        if (permissions.isNotEmpty()) {
            ActivityCompat.requestPermissions(this, permissions.toTypedArray(), PERMISSION_REQUEST_CODE)
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == PERMISSION_REQUEST_CODE) {
            // 权限请求结果处理，可以继续检查更新
            checkForUpdate()
        }
    }

    /**
     * 检查应用更新
     */
    private fun checkForUpdate() {
        // 获取当前应用版本号
        val currentVersion = getCurrentVersionCode()

        // 初始化 UpgradeLink 客户端
        val client = Client(ACCESS_KEY_ID, ACCESS_KEY_SECRET)

        // 创建设备唯一标识
        val deviceId = getOrCreateDeviceId()

        // 构建升级请求
        val urlRequest = UrlUpgradeRequest(
            APP_UNIQUE_ID,
            currentVersion,
            0, // 期望升级版本号，0 表示获取最新版本
            Build.MODEL,
            deviceId
        )

        // 调用 UpgradeLink API 检查更新
        client.getUrlUpgrade(urlRequest, object : Client.Callback<UrlUpgradeResponse> {
            override fun onSuccess(result: UrlUpgradeResponse) {
                runOnUiThread {
                    handleUpgradeResponse(result)
                }
            }

            override fun onFailure(e: Exception) {
                // 升级检查失败，静默处理
                e.printStackTrace()
            }
        })
    }

    /**
     * 处理升级响应
     */
    private fun handleUpgradeResponse(result: UrlUpgradeResponse) {
        // 检查是否需要升级（根据 UpgradeLink 返回的数据判断）
        if (result.data == null || result.data.downloadUrl.isNullOrEmpty()) {
            return // 没有更新或不需要升级
        }

        // 显示升级对话框
        AlertDialog.Builder(this)
            .setTitle(result.msg ?: "发现新版本")
            .setMessage(result.data.promptUpgradeContent ?: "有新的版本可用，是否立即更新？")
            .setCancelable(!result.data.isForceUpgrade) // 强制升级时不可取消
            .setPositiveButton("立即更新") { _, _ ->
                startDownloadUpdate(result.data)
            }
            .apply {
                if (!result.data.isForceUpgrade) {
                    setNegativeButton("稍后再说", null)
                }
            }
            .create()
            .show()
    }

    /**
     * 开始下载更新
     */
    private fun startDownloadUpdate(upgradeData: UrlUpgradeResponse.UrlUpgradeData) {
        val downloadUrl = upgradeData.downloadUrl ?: return

        // 配置下载管理器
        val configuration = UpdateConfiguration()
            .setEnableLog(true)
            .setJumpInstallPage(true)
            .setShowNotification(true)
            .setForcedUpgrade(upgradeData.isForceUpgrade)

        // 创建下载管理器
        val manager = DownloadManager.Builder(this)
            .setApkUrl(downloadUrl)
            .setApkName("stitchwork_update.apk")
            .setConfiguration(configuration)
            .setDownloadListener(object : OnDownloadListener {
                override fun start() {
                    // 开始下载
                }

                override fun downloading(max: Int, progress: Int) {
                    // 下载中
                }

                override fun done(apk: File) {
                    // 下载完成
                }

                override fun cancel() {
                    // 下载取消
                }

                override fun error(e: Throwable) {
                    // 下载出错
                    e.printStackTrace()
                }
            })
            .build()

        // 开始下载
        manager.download()
    }

    /**
     * 获取当前应用版本号
     */
    private fun getCurrentVersionCode(): Int {
        return try {
            val packageInfo = packageManager.getPackageInfo(packageName, 0)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                packageInfo.longVersionCode.toInt()
            } else {
                @Suppress("DEPRECATION")
                packageInfo.versionCode
            }
        } catch (e: Exception) {
            1 // 默认版本号
        }
    }

    /**
     * 获取或创建设备唯一标识
     */
    private fun getOrCreateDeviceId(): String {
        return try {
            // 使用 Android ID 作为设备标识
            Settings.Secure.getString(contentResolver, Settings.Secure.ANDROID_ID)
                ?: "unknown_${System.currentTimeMillis()}"
        } catch (e: Exception) {
            "unknown_${System.currentTimeMillis()}"
        }
    }
}
