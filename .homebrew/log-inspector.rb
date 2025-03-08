class LogInspector < Formula
  desc "AI-powered log analysis tool"
  homepage "https://github.com/hoaihuongbk/log-inspector"
  version "0.1.0"

  url "https://github.com/hoaihuongbk/log-inspector/releases/download/v#{version}/log-inspector"
  sha256 "YOUR_BINARY_SHA256"

  def install
    bin.install "log-inspector"
  end

  test do
    system "#{bin}/log-inspector", "--version"
  end
end
