# Asciinema Setup for Demo Recordings

This guide explains how to set up asciinema.org integration for automated demo recordings.

## Prerequisites

You need an asciinema.org account and API token.

## Step 1: Create Asciinema Account

1. Go to https://asciinema.org
2. Sign up for a free account
3. Verify your email

## Step 2: Get Install ID

1. Install asciinema locally:
   ```bash
   # macOS
   brew install asciinema

   # Ubuntu/Debian
   sudo apt-get install asciinema

   # Or via pip
   pip install asciinema
   ```

2. Authenticate asciinema (this creates a local install ID):
   ```bash
   asciinema auth
   ```

3. Get your Install ID:
   ```bash
   cat ~/.config/asciinema/install-id
   ```

   Copy this ID - you'll need it for GitHub secrets.

   **Alternative:** Go to https://asciinema.org/user/edit and copy the Installation ID shown there.

## Step 3: Add GitHub Secret

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Set:
   - **Name:** `ASCIINEMA_INSTALL_ID`
   - **Value:** [paste your install ID from step 2]
5. Click **Add secret**

**Note:** Asciinema changed from `ASCIINEMA_API_TOKEN` to `ASCIINEMA_INSTALL_ID` in recent versions.

## Step 4: Verify Setup

The secret is now available to GitHub Actions workflows but hidden from public view.

To test:
1. Go to **Actions** tab in GitHub
2. Select **Create Demo Recordings** workflow
3. Click **Run workflow**
4. Wait for completion
5. Check the workflow logs for upload URLs

## Security Notes

- The token is stored as an encrypted GitHub secret
- It's only accessible to GitHub Actions workflows in this repo
- It's never exposed in logs or public files
- Only repository admins can view/edit secrets

## Manual Recording (Local)

To create recordings locally without uploading:

```bash
# Test Flakiness Detector
cd test-flakiness-detector
bash ../scripts/record-flakiness-demo.sh

# CLI Progress Reporting
cd cli-progress-reporting
bash ../scripts/record-progress-demo.sh
```

The `.cast` files will be saved locally. You can upload them manually:

```bash
asciinema upload demo.cast
```

## Workflow Details

The `.github/workflows/create-demos.yml` workflow:
- Runs on manual trigger only (workflow_dispatch)
- Installs asciinema and tool dependencies
- Records terminal sessions with proper timing
- Uploads to asciinema.org using the secret token
- Converts recordings to GIFs (if agg is available)
- Commits demo files to the repository

## Converting to GIF

For GIF generation, install `agg` (asciinema gif generator):

```bash
cargo install --git https://github.com/asciinema/agg
```

Or use the workflow which installs it automatically.
