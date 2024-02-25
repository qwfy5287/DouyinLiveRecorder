from moviepy.editor import VideoFileClip

def split_video_into_chunks(video_path, chunk_length=30):
    # Load the video
    video = VideoFileClip(video_path)
    video_duration = int(video.duration)

    # Calculate the number of chunks
    num_chunks = video_duration // chunk_length + (video_duration % chunk_length > 0)

    # Split the video into chunks
    for i in range(num_chunks):
        start_time = i * chunk_length
        end_time = min((i + 1) * chunk_length, video_duration)
        # Create a subclip for the current chunk
        video_chunk = video.subclip(start_time, end_time)
        # Define the output path for the chunk
        output_path = f"{video_path}_chunk_{i+1}.mp4"
        # Write the video chunk to a file
        video_chunk.write_videofile(output_path, codec='libx264', audio_codec='aac')
        print(f"Chunk {i+1}/{num_chunks} saved to {output_path}")

# Usage
# split_video_into_chunks('path_to_your_video.mp4')
split_video_into_chunks('./data2/朴夫人_2024-02-24_19-40-06_006.mp4')
