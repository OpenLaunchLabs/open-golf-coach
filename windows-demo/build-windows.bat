@echo off
REM Build script for Windows

echo OpenGolfCoach Windows Demo - Build Script
echo ==========================================
echo.

echo Building for Windows (release mode)...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Build failed!
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo Build complete!
echo.
echo Executable location:
echo   target\release\opengolfcoach-windows-demo.exe
echo.
echo The application is ready to run!
echo Double-click the .exe or run from command line.
echo.
pause
