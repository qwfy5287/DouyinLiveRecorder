from moviepy.editor import VideoFileClip
from concurrent.futures import ThreadPoolExecutor
import math

# Function to split a video into chunks
def split_video_into_chunks(filename, chunk_length):
    # Load the video file
    clip = VideoFileClip(filename)

    # Calculate the number of chunks
    duration = clip.duration
    num_chunks = math.ceil(duration / chunk_length)

    # Function to extract and save a chunk
    def extract_chunk(i):
        start_time = i * chunk_length
        end_time = min((i + 1) * chunk_length, duration)
        chunk = clip.subclip(start_time, end_time)
        chunk_filename = f'{filename}_chunk_{i}.mp4'
        chunk.write_videofile(chunk_filename, codec='libx264', preset='fast')

    # Use ThreadPoolExecutor to parallelize the extraction
    with ThreadPoolExecutor(max_workers=4) as executor:
        executor.map(extract_chunk, range(num_chunks))

    # Close the video file
    clip.close()

# Example usage
split_video_into_chunks('./data2/朴夫人_2024-02-24_19-40-06_006.mp4', 12) # Use the method with your video filename and desired chunk length

# Replace the above comment with the actual filename and chunk length as required,
# when running the code outside of this example.
