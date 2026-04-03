import sys
import json
from faster_whisper import WhisperModel

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No file path provided"}))
        sys.exit(1)

    file_path = sys.argv[1]

    try:
        model = WhisperModel("base", device="cpu", compute_type="int8")
        segments, info = model.transcribe(file_path, vad_filter=True)

        text = " ".join(segment.text.strip() for segment in segments)

        print(json.dumps({
            "text": text,
            "language": info.language,
            "language_probability": round(info.language_probability, 3)
        }))

    except FileNotFoundError:
        print(json.dumps({"error": f"File not found: {file_path}"}))
        sys.exit(1)
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()
