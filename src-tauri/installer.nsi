Unicode true

!include "MUI2.nsh"

# アプリケーション定義
!define APP_NAME "notice-ical"
!define APP_EXE "${APP_NAME}.exe"
!define APP_IDENTIFIER "com.mo.noticeical"

# 製品情報定義
!define PRODUCT_NAME "${APP_NAME}"
!define PRODUCT_VERSION "1.0.0"
!define PRODUCT_PUBLISHER "Masashi.Ohashi"
!define PRODUCT_URL "https://github.com/helvetica822/notice-ical"

# レジストリのパス定義
!define REG_KEY_HKCU "Software\Microsoft\Windows\CurrentVersion\Run"
!define REG_KEY_HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"

# 出力されるファイル名
#OutFile "setup_${PRODUCT_NAME}.exe"
OutFile "..\..\bundle\nsis\notice-ical_${PRODUCT_VERSION}_x64-setup.exe"

# デフォルトのインストールパス
InstallDir "$PROGRAMFILES\${APP_NAME}"

# UAC表示
RequestExecutionLevel admin

# アプリケーション情報
!define MUI_PRODUCT "${PRODUCT_NAME}"
!define MUI_VERSION "${PRODUCT_VERSION}"
!define MUI_COPYRIGHT "Copyright © 2025 ${PRODUCT_PUBLISHER}"
!define MUI_URL "${PRODUCT_URL}"
!define MUI_INSTALLDIR "$INSTDIR"

# インストール画面
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

# アンインストール画面
!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!define MUI_UNFINISHPAGE_NOAUTOCLOSE
!insertmacro MUI_UNPAGE_FINISH

# 日本語UI
!insertmacro MUI_LANGUAGE "Japanese"

Section "Install"
    SetOutPath "$INSTDIR"
    File "..\..\notice-ical.exe"

    # 64ビットレジストリを明示
    SetRegView 64

    # なぜか HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\notice-ical に登録される
    # レジストリに自動起動を登録
    WriteRegStr HKCU "${REG_KEY_HKCU}" "${APP_NAME}" "$INSTDIR\${APP_EXE}"

    # レジストリに製品情報を登録(プログラムと機能に表示するため)
    WriteRegStr HKLM "${REG_KEY_HKLM}" "DisplayName" "${PRODUCT_NAME}"
    WriteRegStr HKLM "${REG_KEY_HKLM}" "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "${REG_KEY_HKLM}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr HKLM "${REG_KEY_HKLM}" "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegStr HKLM "${REG_KEY_HKLM}" "URL" "${PRODUCT_URL}"

    # スタートメニューへの登録
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\notice-ical.exe" "" "" "" SW_SHOWNORMAL "" "${APP_IDENTIFIER}"

    # アンインストーラの作成
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    # アプリケーションを起動
    Exec "$INSTDIR\${APP_EXE}"

    Sleep 1000
SectionEnd

Section "Uninstall"
    # インストールしたファイルを削除
    Delete "$INSTDIR\${APP_EXE}"
    Delete "$INSTDIR\Uninstall.exe"
    # フォルダも消すなら
    RMDir "$INSTDIR"

    # レジストリから自動起動の登録を削除
    DeleteRegKey HKCU "${REG_KEY_HKCU}\${APP_NAME}"
    # レジストリから製品情報削除
    DeleteRegKey HKLM "${REG_KEY_HKLM}"
    
    # スタートメニューから削除
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
SectionEnd
