!define APP_NAME "notice-ical"
!define APP_EXE "${APP_NAME}.exe"

!define REG_KEY_HKCU "Software\Microsoft\Windows\CurrentVersion\Run"

!macro NSIS_HOOK_PREINSTALL
!macroend

!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKCU "${REG_KEY_HKCU}" "${APP_NAME}" "$INSTDIR\${APP_EXE}"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  DeleteRegKey HKCU "${REG_KEY_HKCU}\${APP_NAME}"
!macroend
