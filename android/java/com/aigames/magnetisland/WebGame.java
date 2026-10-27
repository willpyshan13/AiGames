package com.aigames.magnetisland;

import android.app.Activity;
import android.app.AlertDialog;
import android.app.Application;
import android.app.Dialog;
import android.graphics.Color;
import android.net.Uri;
import android.os.Bundle;
import android.view.KeyEvent;
import android.view.View;
import android.view.ViewGroup;
import android.view.Window;
import android.view.WindowManager;
import android.webkit.WebChromeClient;
import android.webkit.WebResourceError;
import android.webkit.WebResourceRequest;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.FrameLayout;
import android.widget.Toast;
import java.lang.ref.WeakReference;

/** Full-screen website overlay above the existing Bevy NativeActivity. */
public final class WebGame {
    private static final String URL = "https://www.yikm.net/play?id=4275";
    private static WeakReference<Dialog> active = new WeakReference<>(null);

    public static void open(Activity activity) {
        activity.runOnUiThread(() -> {
            if (activity.isFinishing() || activity.isDestroyed()) return;
            Dialog previous = active.get();
            if (previous != null && previous.isShowing()) return;
            try {
                show(activity);
            } catch (RuntimeException error) {
                android.util.Log.e("WebGame", "Cannot open website", error);
                Toast.makeText(activity, "无法打开游戏网页，请检查系统 WebView。", Toast.LENGTH_LONG).show();
            }
        });
    }

    private static void show(Activity activity) {
        Dialog dialog = new Dialog(activity, android.R.style.Theme_Material_NoActionBar_Fullscreen);
        dialog.requestWindowFeature(Window.FEATURE_NO_TITLE);
        FrameLayout root = new FrameLayout(activity);
        root.setBackgroundColor(Color.BLACK);
        WebView web = new WebView(activity);
        web.setBackgroundColor(Color.BLACK);
        WebSettings settings = web.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setUseWideViewPort(true);
        settings.setLoadWithOverviewMode(true);
        settings.setAllowFileAccess(false);
        settings.setAllowContentAccess(false);
        settings.setMixedContentMode(WebSettings.MIXED_CONTENT_NEVER_ALLOW);
        // No JavaScript bridge or page rewriting: use the site's own controls.
        FrameLayout.LayoutParams fill = new FrameLayout.LayoutParams(-1, -1);
        root.addView(web, fill);
        dialog.setContentView(root);
        final AlertDialog[] errorNotice = new AlertDialog[1];

        WebChromeClient chrome = new WebChromeClient() {
            private View customView;
            private CustomViewCallback callback;
            @Override public void onShowCustomView(View view, CustomViewCallback cb) {
                if (customView != null) { cb.onCustomViewHidden(); return; }
                customView = view;
                callback = cb;
                web.setVisibility(View.GONE);
                root.addView(view, new FrameLayout.LayoutParams(-1, -1));
                immersive(dialog.getWindow());
            }
            @Override public void onHideCustomView() {
                if (customView == null) return;
                root.removeView(customView);
                customView = null;
                web.setVisibility(View.VISIBLE);
                if (callback != null) callback.onCustomViewHidden();
                callback = null;
            }
        };
        web.setWebChromeClient(chrome);
        web.setWebViewClient(new WebViewClient() {
            private boolean showingError;
            @Override public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {
                if (!request.isForMainFrame()) return false;
                Uri uri = request.getUrl();
                String host = uri.getHost();
                // Keep the game inside the app; do not launch ad destinations
                // or external app/payment schemes from this children's app.
                return !("https".equals(uri.getScheme()) && host != null
                    && (host.equals("yikm.net") || host.endsWith(".yikm.net")));
            }
            @Override public void onReceivedError(WebView view, WebResourceRequest request, WebResourceError error) {
                if (!request.isForMainFrame() || showingError || !dialog.isShowing()) return;
                showingError = true;
                AlertDialog notice = new AlertDialog.Builder(activity)
                    .setTitle("游戏网页暂时无法打开")
                    .setMessage("请检查网络连接后重试。")
                    .setPositiveButton("重试", (d, which) -> web.loadUrl(URL))
                    .setNegativeButton("返回游戏库", (d, which) -> dialog.dismiss())
                    .create();
                notice.setOnDismissListener(d -> showingError = false);
                errorNotice[0] = notice;
                notice.show();
            }
        });
        Application.ActivityLifecycleCallbacks lifecycle = new Application.ActivityLifecycleCallbacks() {
            public void onActivityPaused(Activity a) { if (a == activity) web.onPause(); }
            public void onActivityResumed(Activity a) { if (a == activity) web.onResume(); }
            public void onActivityDestroyed(Activity a) { if (a == activity) dialog.dismiss(); }
            public void onActivityCreated(Activity a, Bundle b) {}
            public void onActivityStarted(Activity a) {}
            public void onActivityStopped(Activity a) {}
            public void onActivitySaveInstanceState(Activity a, Bundle b) {}
        };
        dialog.setOnKeyListener((d, key, event) -> {
            if (key != KeyEvent.KEYCODE_BACK) return false;
            if (event.getAction() == KeyEvent.ACTION_UP) dialog.dismiss();
            return true;
        });
        dialog.setOnDismissListener(d -> {
            if (errorNotice[0] != null) errorNotice[0].dismiss();
            activity.getApplication().unregisterActivityLifecycleCallbacks(lifecycle);
            chrome.onHideCustomView();
            web.stopLoading();
            web.onPause();
            root.removeView(web);
            web.destroy();
            active.clear();
        });
        activity.getApplication().registerActivityLifecycleCallbacks(lifecycle);
        dialog.show();
        Window window = dialog.getWindow();
        if (window != null) {
            window.setLayout(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT);
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
            immersive(window);
            window.getDecorView().setOnSystemUiVisibilityChangeListener(visibility -> {
                if ((visibility & View.SYSTEM_UI_FLAG_FULLSCREEN) == 0) immersive(window);
            });
        }
        active = new WeakReference<>(dialog);
        web.loadUrl(URL);
    }

    private static void immersive(Window window) {
        if (window == null) return;
        window.getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_FULLSCREEN
            | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
            | View.SYSTEM_UI_FLAG_LAYOUT_STABLE | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
            | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION);
    }
}
