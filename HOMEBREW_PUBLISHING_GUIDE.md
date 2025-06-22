# Publishing jpeek to Homebrew

This guide will help you publish your `jpeek` tool to Homebrew so users can install it with `brew install jpeek`.

## Prerequisites

1. ✅ **Published to crates.io** - Your crate is already published at https://crates.io/crates/jpeek
2. ✅ **GitHub repository** - Your code is hosted at https://github.com/morshedul-munna/jpeek
3. ✅ **MIT License** - Your project is MIT licensed
4. ✅ **Stable release** - You have a v0.1.0 release

## Step 1: Create a Homebrew Tap (Recommended)

The easiest way to publish to Homebrew is to create your own tap. This allows you to maintain your formula independently.

### 1.1 Create a new repository for your tap

Create a new GitHub repository named `homebrew-tap` (or `homebrew-jpeek`).

### 1.2 Add the formula to your tap

Copy the `jpeek.rb` file from this directory to your tap repository in the root directory.

### 1.3 Test your formula locally

```bash
# Clone your tap repository
git clone https://github.com/yourusername/homebrew-tap.git
cd homebrew-tap

# Test the formula
brew install --build-from-source ./jpeek.rb

# Test that it works
jpeek --version
```

### 1.4 Users can install from your tap

Once published, users can install your tool with:

```bash
brew tap yourusername/tap
brew install jpeek
```

## Step 2: Submit to Homebrew Core (Alternative)

If you want your tool to be available directly with `brew install jpeek` (without tapping), you can submit it to the main Homebrew repository.

### 2.1 Fork the Homebrew repository

1. Go to https://github.com/Homebrew/homebrew-core
2. Fork the repository

### 2.2 Create a pull request

1. Clone your fork
2. Create a new branch
3. Add the `jpeek.rb` file to the `Formula/` directory
4. Commit and push your changes
5. Create a pull request

### 2.2.1 Formula location

The formula should be placed in:

```
homebrew-core/Formula/j/jpeek.rb
```

### 2.2.2 Update the formula path

Update the formula to use the correct path:

```ruby
class Jpeek < Formula
  desc "A simple tool to peek into your JWT token & Decode it"
  homepage "https://github.com/morshedul-munna/jpeek"
  url "https://static.crates.io/crates/jpeek/jpeek-0.1.0.crate"
  sha256 "7cd150966d575a54455c5cb0ef5f1ced487ef335b9fa90952a7c733ca7753332"
  license "MIT"
  head "https://github.com/morshedul-munna/jpeek.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary works
    system "#{bin}/jpeek", "--version"
  end
end
```

## Step 3: Testing Your Formula

Before submitting, test your formula thoroughly:

```bash
# Test installation
brew install --build-from-source ./jpeek.rb

# Test uninstallation
brew uninstall jpeek

# Test the binary
jpeek --version
jpeek --help
```

## Step 4: Updating for New Versions

When you release a new version:

1. Update the version number in the formula
2. Update the URL to point to the new crate version
3. Calculate and update the SHA256 hash
4. Test the formula
5. Submit the changes

### Calculate new SHA256:

```bash
curl -sL https://static.crates.io/crates/jpeek/jpeek-NEW_VERSION.crate | shasum -a 256
```

## Requirements for Homebrew Core

If submitting to Homebrew Core, your project must meet these criteria:

- ✅ **Open source** - Your code is publicly available
- ✅ **Stable releases** - You have tagged releases
- ✅ **Documentation** - You have a README with usage examples
- ✅ **Tests** - Your formula includes a test
- ✅ **License** - You have an OSI-approved license
- ✅ **Maintenance** - You're actively maintaining the project

## Recommended Approach

For a new tool like `jpeek`, I recommend starting with **Step 1 (Homebrew Tap)** because:

1. **Faster approval** - No need to wait for Homebrew maintainers
2. **Full control** - You can update whenever you want
3. **Easier maintenance** - You manage your own formula
4. **Community building** - Users can star and follow your tap

Once your tool gains popularity and you have more releases, you can then consider submitting to Homebrew Core.

## Next Steps

1. Create your `homebrew-tap` repository
2. Add the `jpeek.rb` formula
3. Test it locally
4. Push to GitHub
5. Share the installation instructions with your users

## Installation Instructions for Users

Once published, users can install with:

```bash
# If using your own tap
brew tap yourusername/tap
brew install jpeek

# Or if accepted to Homebrew Core
brew install jpeek
```

## Useful Commands

```bash
# Test formula locally
brew install --build-from-source ./jpeek.rb

# Check formula syntax
brew audit --strict ./jpeek.rb

# Update formula for new version
brew bump-formula-pr --url=https://static.crates.io/crates/jpeek/jpeek-NEW_VERSION.crate jpeek
```
