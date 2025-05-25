# MITRE ATT&CK Simulation Tool

This project simulates selected adversary techniques from the
[MITRE ATT&CK](https://attack.mitre.org/) framework in a safe and controlled Linux environment.  
It is intended for security testing, detection engineering, and cybersecurity education.

## 🔍 What It Does

- Simulates benign versions of common attack behaviors
- Logs actions performed, including commands and timestamps
- Can be used to verify if detection systems (e.g., auditd, syslog) are correctly configured

## 🎯 Simulated Techniques

| Tactic               | Technique ID | Technique Name                  |
|----------------------|--------------|----------------------------------|
| Discovery            | T1087        | Account Discovery               |
| Persistence          | T1543.003    | Create or Modify Systemd Service |
| Execution            | T1059.004    | User Execution via Bash Script  |

> All simulations are non-destructive and focus on realistic but safe approximations of attacker behavior.

## 📁 Output

Each simulation generates:
- A structured log with actions, timestamps, and metadata
- Optional integration with system logging tools for detection validation
