# DocGen Distribution Guide for GreyBeard Outsourcing

## 🏢 Company-Wide Deployment Instructions

### Prerequisites for IT Department

1. **Azure DevOps Access**
   - Repository: `https://GreybeardTrilogy@dev.azure.com/GreybeardTrilogy/Document%20Generation/_git/Document%20Generation`
   - Ensure all team members have access to the repository

2. **Build Environment**
   - Windows Server with PowerShell 5.1+
   - Linux server with Bash
   - Rust toolchain installed
   - Git for source control

3. **Network Requirements**
   - PowerShell Remoting enabled on Windows machines
   - SSH access to Linux machines
   - Shared network drive for distribution (optional)

## 🚀 Deployment Methods

### Method 1: Individual Installation (Recommended for Developers)

**Windows Users:**
```powershell
# Clone the repository
git clone "https://GreybeardTrilogy@dev.azure.com/GreybeardTrilogy/Document%20Generation/_git/Document%20Generation"
cd "Document%20Generation"  # Note: Directory name will be URL-encoded

# Run installation script
.\install.ps1
```

**Linux Users:**
```bash
# Clone the repository
git clone "https://GreybeardTrilogy@dev.azure.com/GreybeardTrilogy/Document%20Generation/_git/Document%20Generation"
cd "Document%20Generation"  # Note: Directory name will be URL-encoded

# Run installation script
./install.sh
```

### Method 2: Mass Deployment (IT Department)

**For Windows Networks:**
```powershell
# Build the project
cargo build --release

# Create computer list file
@"
WORKSTATION-001
WORKSTATION-002
DEV-MACHINE-01
"@ | Out-File -FilePath computers.txt

# Deploy to all machines
.\deploy.ps1 -ComputerListFile computers.txt
```

**For Linux Networks:**
```bash
# Build the project
cargo build --release

# Deploy using Ansible (create playbook)
ansible-playbook -i inventory deploy-docgen.yml
```

### Method 3: Pre-built Binaries

1. Download from Azure DevOps releases
2. Extract to appropriate directory
3. Add to system PATH
4. Distribute via Group Policy (Windows) or configuration management

## 📋 Installation Verification

After installation, verify with:
```bash
docgen version
# Should output: DocGen v1.0.0

docgen config
# Should show user configuration status
```

## 🔧 Configuration Management

### User Configuration
Each user needs to configure their information once:
```bash
docgen generate
# Press Ctrl+E to configure user info
```

### Centralized Configuration (Optional)
Create a company-wide default configuration:

**Windows:** `%PROGRAMDATA%\GreyBeard\DocGen\default_config.json`
**Linux:** `/etc/docgen/default_config.json`

```json
{
    "company": "GreyBeard Outsourcing",
    "default_settings": {
        "api_endpoint": "https://api.greybeard.internal/docgen",
        "cost_currency": "PKR",
        "exchange_rate": 280
    }
}
```

## 🐛 Troubleshooting

### Common Issues

1. **"Command not found" after installation**
   - Restart terminal/command prompt
   - Check PATH environment variable
   - Re-run installation script

2. **Git integration not working**
   - Ensure Git is installed and in PATH
   - Verify repository has git history
   - Check git repository status

3. **Permission errors on Windows**
   - Run PowerShell as Administrator
   - Check Windows Defender exclusions
   - Verify user has write permissions

4. **Network connectivity issues**
   - Check firewall settings
   - Verify internet connectivity for AI features
   - Test API endpoints if using internal services

### Support Escalation

**Level 1 Support:**
- Check installation status: `docgen version`
- Verify user configuration: `docgen config`
- Restart application and retry

**Level 2 Support (IT Department):**
- Check system logs
- Verify network connectivity
- Reinstall with elevated privileges

**Level 3 Support (Development Team):**
- Contact: Omer Jauhar <omer.jauhar@greybeardsupport.com>
- Include: OS version, error messages, log files
- Provide: Steps to reproduce the issue

## 📊 Usage Analytics (Optional)

To track company-wide usage:

1. **Central Logging Server**
   - Configure rsyslog or Windows Event Forwarding
   - Collect DocGen usage statistics
   - Monitor error rates and performance

2. **Usage Metrics**
   ```bash
   # Add to crontab for periodic reporting
   0 9 * * 1 /usr/local/bin/generate_docgen_report.sh
   ```

## 🔒 Security Considerations

1. **Access Control**
   - Limit repository access to authorized personnel
   - Use service accounts for automated deployments
   - Regular access reviews

2. **Data Protection**
   - User configurations stored locally
   - No code data transmitted without explicit consent
   - API communications encrypted (HTTPS)

3. **Compliance**
   - Audit trail for all installations
   - Version control for configuration changes
   - Regular security updates

## 📅 Maintenance Schedule

**Weekly:**
- Check for security updates
- Monitor error reports
- Update documentation

**Monthly:**
- Review usage statistics
- Update deployment scripts
- Backup configurations

**Quarterly:**
- Major version updates
- Security audits
- Performance reviews

## 🎯 Success Metrics

Track these KPIs for DocGen adoption:

1. **Installation Rate**
   - Target: 90% of development team within 30 days
   - Metric: Active installations / Total developers

2. **Usage Frequency**
   - Target: Daily usage by 70% of users
   - Metric: Daily active users / Total installations

3. **Error Rate**
   - Target: <5% of sessions result in errors
   - Metric: Error sessions / Total sessions

4. **Support Tickets**
   - Target: <10 tickets per month after initial rollout
   - Metric: Monthly support requests

## 📞 Contact Information

**Maintainer:** Omer Jauhar  
**Email:** omer.jauhar@greybeardsupport.com  
**Department:** Software Development  
**Internal Extension:** [Your extension]  

**IT Support:** [IT Department contact]  
**Emergency Contact:** [Manager contact]  

---

*This document should be updated with each major release and reviewed quarterly.*
