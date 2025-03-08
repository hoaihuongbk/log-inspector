class LogInspector < Formula
  desc "AI-powered log analysis tool"
  homepage "https://github.com/hoaihuongbk/log-inspector"
  version "0.1.1"

  url "https://github.com/hoaihuongbk/log-inspector/releases/download/v#{version}/log-inspector"
  sha256 "8e4f4fe68756b2adabf87f8dd1b59e30deacd14ffb500c1d8dbb4651fc2c9f1e"

  def install
    bin.install "log-inspector"
  end

  test do
    system "#{bin}/log-inspector", "--version"
  end
end
