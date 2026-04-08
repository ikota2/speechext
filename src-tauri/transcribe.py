import sys
import json
import os
import argparse
from pathlib import Path
from dotenv import load_dotenv

load_dotenv(Path(__file__).resolve().parent.parent / ".env")

ENGINE = os.environ.get("TRANSCRIBE_ENGINE", "local")


def transcribe_local(file_paths, language):
    from faster_whisper import WhisperModel

    total = len(file_paths)
    model = WhisperModel("base", device="cpu", compute_type="int8")

    segments_all = []
    detected_language = None
    detected_language_prob = None

    for i, file_path in enumerate(file_paths):
        progress = {"progress": i + 1, "total": total, "file": os.path.basename(file_path)}
        print(json.dumps(progress), file=sys.stderr, flush=True)

        try:
            segments, info = model.transcribe(file_path, language=language, vad_filter=True, beam_size=1)
            text = " ".join(seg.text.strip() for seg in segments).strip()

            if detected_language is None:
                detected_language = info.language
                detected_language_prob = round(info.language_probability, 3)

            if text:
                segments_all.append({
                    "index": i,
                    "file": os.path.basename(file_path),
                    "text": text
                })

        except Exception as e:
            segments_all.append({
                "index": i,
                "file": os.path.basename(file_path),
                "text": "",
                "error": str(e)
            })

    full_text = " ".join(s["text"] for s in segments_all if s.get("text"))

    return {
        "language": detected_language,
        "language_probability": detected_language_prob,
        "total_files": total,
        "segments": segments_all,
        "full_text": full_text
    }


def transcribe_groq(file_paths, language):
    from groq import Groq

    api_key = os.environ.get("GROQ_API_KEY")
    if not api_key:
        return {"error": "GROQ_API_KEY not set"}

    client = Groq(api_key=api_key)
    total = len(file_paths)

    segments_all = []
    detected_language = None
    detected_language_prob = None

    for i, file_path in enumerate(file_paths):
        progress = {"progress": i + 1, "total": total, "file": os.path.basename(file_path)}
        print(json.dumps(progress), file=sys.stderr, flush=True)

        try:
            with open(file_path, "rb") as f:
                response = client.audio.transcriptions.create(
                    model="whisper-large-v3",
                    file=f,
                    language=language,
                    response_format="verbose_json"
                )

            text = response.text.strip()

            if detected_language is None:
                detected_language = response.language
                detected_language_prob = 1.0

            if text:
                segments_all.append({
                    "index": i,
                    "file": os.path.basename(file_path),
                    "text": text
                })

        except Exception as e:
            segments_all.append({
                "index": i,
                "file": os.path.basename(file_path),
                "text": "",
                "error": str(e)
            })

    full_text = " ".join(s["text"] for s in segments_all if s.get("text"))

    return {
        "language": detected_language,
        "language_probability": detected_language_prob,
        "total_files": total,
        "segments": segments_all,
        "full_text": full_text
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lang", required=True, choices=["en", "es", "ru"])
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()

    for path in args.files:
        if not os.path.isfile(path):
            print(json.dumps({"error": f"File not found: {path}"}))
            sys.exit(1)

    if ENGINE == "groq":
        result = transcribe_groq(args.files, args.lang)
    else:
        result = transcribe_local(args.files, args.lang)

    print(json.dumps(result, ensure_ascii=False))


if __name__ == "__main__":
    main()
