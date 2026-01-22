# Enabling GitHub Pages for Documentation

Follow these steps to enable public documentation hosting:

1. **Go to your repository settings:**
   - Navigate to: https://github.com/rm-ritik/Helion/settings/pages

2. **Configure GitHub Pages:**
   - Under "Build and deployment"
   - Source: Select "GitHub Actions"
   - Click "Save"

3. **Push the workflow:**
   ```bash
   git add .github/workflows/docs.yml README.md
   git commit -m "docs: Add automatic documentation deployment via GitHub Pages"
   git push origin main
   ```

4. **Wait for deployment:**
   - Go to Actions tab: https://github.com/rm-ritik/Helion/actions
   - The "Deploy Documentation" workflow will run automatically
   - Takes ~2-3 minutes

5. **Access your docs:**
   Once deployed, documentation will be available at:
   https://rm-ritik.github.io/Helion/

## Automatic Updates

Every time you push to `main` branch, documentation will automatically rebuild and deploy!

## Local Preview

To preview docs locally before pushing:
```bash
cd core
cargo doc --no-deps --open
```
