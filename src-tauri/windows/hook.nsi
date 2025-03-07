!macro NSIS_HOOK_PREINSTALL
    DetailPrint "$(removingFirewallRule)"
    nsExec::ExecToLog 'netsh advfirewall firewall delete rule name="${PRODUCTNAME}"'
    
    DetailPrint "$(addingFirewallRule)"
    nsExec::ExecToLog 'netsh advfirewall firewall add rule name="${PRODUCTNAME}" dir=in action=allow program="$INSTDIR\${MAINBINARYNAME}.exe" enable=yes description="Allow ${PRODUCTNAME} to communicate on local network"'
    nsExec::ExecToLog 'netsh advfirewall firewall add rule name="${PRODUCTNAME}" dir=out action=allow program="$INSTDIR\${MAINBINARYNAME}.exe" enable=yes description="Allow ${PRODUCTNAME} to communicate on local network"'
!macroend

!macro NSIS_HOOK_POSTINSTALL
!macroend

!macro NSIS_HOOK_PREUNINSTALL
    DetailPrint "$(removingFirewallRule)"
    nsExec::ExecToLog 'netsh advfirewall firewall delete rule name="${PRODUCTNAME}"'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
!macroend