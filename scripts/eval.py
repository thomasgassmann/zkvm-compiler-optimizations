import os
import shutil
from cairosvg import svg2png
from io import BytesIO
from PIL import Image
import asyncio
import matplotlib.pyplot as plt

a = "/home/thomas/Downloads/k/out/criterion"
# a = "/run/user/1000/gvfs/sftp:host=prove1/home/user/git/thesis/target/criterion"
# a = "/run/user/1000/gvfs/sftp:host=prove2/home/user/git/thesis/target/criterion"
b = "/home/thomas/git/thesis/benchmarks/results/bench"

target = "/home/thomas/git/thesis/benchmarks/results/bench-rerun"

moves = []
for current_dir in os.listdir(a):
    remote_path = os.path.join(a, current_dir)
    existing_path = os.path.join(b, current_dir)
    local_path = os.path.join(target, current_dir)
    remote_violin = os.path.join(remote_path, "report/violin.svg")
    existing_violin = os.path.join(existing_path, "report/violin.svg")
    if os.path.exists(remote_violin) and os.path.exists(existing_violin):

        def svg_to_image(svg_file):
            png_data = BytesIO()
            with open(svg_file, "r") as f:
                svg_content = f.read()
            svg2png(bytestring=svg_content.encode("utf-8"), write_to=png_data)
            png_data.seek(0)
            return Image.open(png_data)

        remote_img = svg_to_image(remote_violin)
        existing_img = svg_to_image(existing_violin)

        fig, axes = plt.subplots(1, 2, figsize=(10, 5))
        axes[0].imshow(remote_img)
        axes[0].set_title("Remote Violin")
        axes[0].axis("off")

        axes[1].imshow(existing_img)
        axes[1].set_title("Existing Violin")
        axes[1].axis("off")

        plt.tight_layout()
        plt.show()

        res = input("Press 'y' to accept: ")
        if res.lower() == "q":
            print("Exiting...")
            break

        if res.lower() == "d":
            print(f"Deleting remote directory: {remote_path}")
            shutil.rmtree(remote_path)
            continue

        if res.lower() != "y":
            continue

        moves.append((remote_path, local_path))


async def move_directory(remote_path, local_path):
    print(f"Moving from {remote_path} to {local_path}")
    loop = asyncio.get_event_loop()
    await loop.run_in_executor(None, shutil.move, remote_path, local_path)


async def main():
    tasks = [
        move_directory(remote_path, local_path) for remote_path, local_path in moves
    ]
    await asyncio.gather(*tasks)


asyncio.run(main())
