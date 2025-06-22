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