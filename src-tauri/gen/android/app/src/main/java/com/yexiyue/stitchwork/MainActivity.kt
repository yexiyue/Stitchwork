package com.yexiyue.stitchwork

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    // Handle keyboard (IME) insets for Edge-to-Edge mode:
    // When keyboard appears, add bottom padding to push WebView content up;
    // when keyboard hides, remove padding (safe area handled by CSS).
    val contentView = findViewById<android.view.View>(android.R.id.content)
    ViewCompat.setOnApplyWindowInsetsListener(contentView) { view, insets ->
      val imeVisible = insets.isVisible(WindowInsetsCompat.Type.ime())
      val imeHeight = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
      view.setPadding(0, 0, 0, if (imeVisible) imeHeight else 0)
      insets
    }

    // Check for app updates on launch
    UpdateHelper.checkUpdate(this)
  }
}
