package com.yexiyue.stitchwork

import android.os.Bundle
import android.view.View
import android.view.ViewGroup
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  private var webViewRef: WebView? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    val contentView = findViewById<View>(android.R.id.content)

    // Handle keyboard (IME) insets for Edge-to-Edge mode
    ViewCompat.setOnApplyWindowInsetsListener(contentView) { view, insets ->
      val imeVisible = insets.isVisible(WindowInsetsCompat.Type.ime())
      val imeHeight = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
      view.setPadding(0, 0, 0, if (imeVisible) imeHeight else 0)
      insets
    }

    // Check for app updates on launch
    UpdateHelper.checkUpdate(this)

    // Expose JS bridge for manual update check from WebView
    setupJsBridge(contentView)
  }

  private fun setupJsBridge(contentView: View) {
    contentView.post {
      val webView = findWebView(contentView)
      if (webView != null) {
        webViewRef = webView
        webView.addJavascriptInterface(NativeBridge(), "NativeBridge")
      } else {
        contentView.postDelayed({ setupJsBridge(contentView) }, 100)
      }
    }
  }

  private fun findWebView(view: View): WebView? {
    if (view is WebView) return view
    if (view is ViewGroup) {
      for (i in 0 until view.childCount) {
        val result = findWebView(view.getChildAt(i))
        if (result != null) return result
      }
    }
    return null
  }

  inner class NativeBridge {
    @JavascriptInterface
    fun checkUpdate() {
      UpdateHelper.checkUpdate(this@MainActivity) { hasUpdate ->
        if (!hasUpdate) {
          webViewRef?.post {
            webViewRef?.evaluateJavascript(
              "window.__onUpdateCheckResult && window.__onUpdateCheckResult(false)",
              null
            )
          }
        }
      }
    }
  }
}
