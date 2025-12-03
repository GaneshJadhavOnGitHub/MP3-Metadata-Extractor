/// Program to print MetaData of MP3 file.

use std::env;
use mp3_metadata;
use mp3_metadata::MP3Metadata;

const NOT_AVAILABLE: &str = "Not Available";

/// Prints Tags in MP3 file.
fn print_tags(mp3_meta_data: &MP3Metadata){
    
    if let Some(tag) = &mp3_meta_data.tag {
        println!("\n\n========== TAGS ==========");
        println!("Title:      {}", tag.title);
        println!("Artist:     {}", tag.artist);
        println!("Album:      {}", tag.album);
        println!("Year:       {}", tag.year);
        println!("Comment:    {}", tag.comment);
        println!("Genre:      {:?}", tag.genre);
    } else {
        println!("\n\nTags are not present.");
    }
}

/// Prints Information of First Frame in MP3 file.
fn print_first_frame(mp3_meta_data: &MP3Metadata){
    println!("\nNumber of Frames: {}", mp3_meta_data.frames.len());
    if mp3_meta_data.frames.len() != 0 {
        println!("\nShowing  First Frame's Information:");
        for frame in mp3_meta_data.frames[0..1].iter() {
            println!("\n========== FIRST FRAME ==========");
            println!("Size:               {}", frame.size);
            println!("Version:            {:?}", frame.version);
            println!("Layer:              {:?}", frame.layer);
            println!("CRC:                {:?}", frame.crc);
            println!("Bitrate:            {} Kb/s", frame.bitrate);
            println!("Sampling Frequency: {} Hz", frame.sampling_freq);
            println!("Padding:            {}", frame.padding);
            println!("Private Bit:        {}", frame.private_bit);
            println!("Channel Type:       {:?}", frame.chan_type);
            println!("Intensity Stereo:   {}", frame.intensity_stereo);
            println!("MS Stereo:          {}", frame.ms_stereo);
            println!("Copyright:          {:?}", frame.copyright);
            println!("Status:             {:?}", frame.status);
            println!("Emphasis:           {:?}", frame.emphasis);
            println!("Duration:           {:?}", frame.duration.to_owned().unwrap());
            println!("Position:           {:?}", frame.position);
            println!("Offset:             {:?}", frame.offset);
        
      } // End of For
    } // End of if
    else {
        println!("\n\nFrames are not present.");
    }
}


/* 
/// Prints Optional Information in MP3 file. 
fn print_optional_information(mp3_meta_data: &MP3Metadata){
    
    
    if !mp3_meta_data.optional_info.is_empty(){
        println!("\n========== OPTIONAL INFORMATION ==========");
        for optional_audio_tags in mp3_meta_data.optional_info[0..1].iter() {
            println!("Position:                                 {}", optional_audio_tags.position);
            println!("Major Version:                            {}", optional_audio_tags.major_version);
            println!("Minor Version:                            {}", optional_audio_tags.minor_version);
            // Inner if starts
            if optional_audio_tags.album_movie_show.is_some(){
            println!("Album Movie Show:                         {}", optional_audio_tags.album_movie_show.to_owned().unwrap());
            }
            else{
            println!("Album Movie Show:                         {}", NOT_AVAILABLE);
            }
            if optional_audio_tags.bpm.is_some(){
            println!("Beats Per Minute:                         {}", optional_audio_tags.bpm.to_owned().unwrap());
            }
            else{
            println!("Beats Per Minute:                         {}", NOT_AVAILABLE);
            }
           if !&optional_audio_tags.composers.is_empty(){
              for composer in &optional_audio_tags.composers {
            println!("Composer:                                 {}", *composer);
              }
           }
           else{
            println!("Composer:                                 {}", NOT_AVAILABLE);
        }
           if !&optional_audio_tags.content_type.is_empty(){
            for genre in &optional_audio_tags.content_type {
            println!("Genre:                                    {:?}", genre);
            }
         }
         else{
            println!("Genre:                                    {}", NOT_AVAILABLE);
     }
         if optional_audio_tags.copyright.is_some(){
            println!("Copyright:                                {}", optional_audio_tags.copyright.to_owned().unwrap());
            }
        else{
            println!("Copyright:                                {}", NOT_AVAILABLE);
        }  
        if optional_audio_tags.date.is_some(){
            println!("Date:                                     {}", optional_audio_tags.date.to_owned().unwrap());
            }
            else{
            println!("Date:                                     {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.playlist_delay.is_some(){
            println!("Playlist Delay:                           {}", optional_audio_tags.playlist_delay.to_owned().unwrap());
            }
            else{
            println!("Playlist Delay:                           {}", NOT_AVAILABLE);
            }            
       if optional_audio_tags.encoded_by.is_some(){
            println!("Encoded By:                               {}", optional_audio_tags.encoded_by.to_owned().unwrap());
            }
            else{
            println!("Encoded By:                               {}", NOT_AVAILABLE);
            }
       if !&optional_audio_tags.text_writers.is_empty(){
              for text_writers in &optional_audio_tags.text_writers {
            println!("Text Writers:                             {}", *text_writers);
              }
           }
           else{
            println!("Text Writers:                             {}", NOT_AVAILABLE);
        }
        if optional_audio_tags.file_type.is_some(){
            println!("File Type:                                {}", optional_audio_tags.file_type.to_owned().unwrap());
            }
            else{
            println!("File Type:                                {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.time.is_some(){
            println!("Time:                                     {}", optional_audio_tags.time.to_owned().unwrap());
            }
            else{
            println!("Time:                                     {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.content_group_description.is_some(){
            println!("Content Group Description:                {}", optional_audio_tags.content_group_description.to_owned().unwrap());
            }
            else{
            println!("Content Group Description:                {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.subtitle_refinement_description.is_some(){
            println!("Subtitle Refinement Description:          {}", optional_audio_tags.subtitle_refinement_description.to_owned().unwrap());
            }
            else{
            println!("Subtitle Refinement Description:          {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.title.is_some(){
            println!("Title:                                    {}", optional_audio_tags.title.to_owned().unwrap());
            }
            else{
            println!("Title:                                    {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.initial_key.is_some(){
            println!("Initial Key:                              {}", optional_audio_tags.initial_key.to_owned().unwrap());
            }
            else{
            println!("Initial Key:                              {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.language.is_some(){
            println!("Language:                                 {}", optional_audio_tags.language.to_owned().unwrap());
            }
            else{
            println!("Language:                                 {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.length.is_some(){
            println!("Length:                                   {}", optional_audio_tags.length.to_owned().unwrap());
            }
            else{
            println!("Length:                                   {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.media_type.is_some(){
            println!("Media Type:                               {}", optional_audio_tags.media_type.to_owned().unwrap());
            }
            else{
            println!("Media Type:                               {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.original_album_move_show_title.is_some(){
            println!("Original Album Movie Show Title:          {}", optional_audio_tags.original_album_move_show_title.to_owned().unwrap());
            }
            else{
            println!("Original Album Movie Show Title:          {}", NOT_AVAILABLE);
            }
        if optional_audio_tags.original_filename.is_some(){
            println!("Original Filename:                        {}", optional_audio_tags.original_filename.to_owned().unwrap());
            }
            else{
            println!("Original Filename:                        {}", NOT_AVAILABLE);
            }
        if !&optional_audio_tags.original_text_writers.is_empty(){
              for original_text_writers in &optional_audio_tags.original_text_writers {
            println!("Original Text Writers:                    {}", *original_text_writers);
              }
           }
           else{
            println!("Original Text Writers:                    {}", NOT_AVAILABLE);
        }
        if !&optional_audio_tags.original_artists.is_empty(){
            for original_artists in &optional_audio_tags.original_artists {
            println!("Original Artists:                         {}", *original_artists);
            }
         }
         else{
            println!("Original Artists:                         {}", NOT_AVAILABLE);
      }
        if optional_audio_tags.original_release_year.is_some(){
            println!("Original Release Year:                    {}", optional_audio_tags.original_release_year.to_owned().unwrap());
        }
        else{
            println!("Original Release Year:                    {}", NOT_AVAILABLE);
        }
        if optional_audio_tags.file_owner.is_some(){
            println!("File Owner:                               {}", optional_audio_tags.file_owner.to_owned().unwrap());
        }
        else{
            println!("File Owner:                               {}", NOT_AVAILABLE);
        }
        if !&optional_audio_tags.performers.is_empty(){
            for performers in &optional_audio_tags.performers {
            println!("Performers:                               {}", *performers);
            }
         }
         else{
            println!("Performers:                               {}", NOT_AVAILABLE);
        }
        if optional_audio_tags.band.is_some(){
            println!("Band:                                     {}", optional_audio_tags.band.to_owned().unwrap());
        }
        else{
            println!("Band:                                     {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.conductor.is_some(){
            println!("Conductor:                                {}", optional_audio_tags.conductor.to_owned().unwrap());
        }
        else{
            println!("Conductor:                                {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.interpreted.is_some(){
            println!("Interpreted:                              {}", optional_audio_tags.interpreted.to_owned().unwrap());
        }
        else{
            println!("Interpreted:                              {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.part_of_a_set.is_some(){
            println!("Part Of A Set:                            {}", optional_audio_tags.part_of_a_set.to_owned().unwrap());
        }
        else{
            println!("Part Of A Set:                            {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.publisher.is_some(){
            println!("Publisher:                                {}", optional_audio_tags.publisher.to_owned().unwrap());
        }
        else{
            println!("Publisher:                                {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.track_number.is_some(){
            println!("Track Number:                             {}", optional_audio_tags.track_number.to_owned().unwrap());
        }
        else{
            println!("Track Number:                             {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.recording_dates.is_some(){
            println!("Recording Dates:                          {}", optional_audio_tags.recording_dates.to_owned().unwrap());
        }
        else{
            println!("Recording Dates:                          {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.internet_radio_station_name.is_some(){
            println!("Internet Radio Station Name:              {}", optional_audio_tags.internet_radio_station_name.to_owned().unwrap());
        }
        else{
            println!("Internet Radio Station Name:              {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.internet_radio_station_owner.is_some(){
            println!("Internet Radio Station Owner:             {}", optional_audio_tags.internet_radio_station_owner.to_owned().unwrap());
        }
        else{
            println!("Internet Radio Station Owner:             {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.size.is_some(){
            println!("Size:                                     {}", optional_audio_tags.size.to_owned().unwrap());
        }
        else{
            println!("Size:                                     {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.international_standard_recording_code.is_some(){
            println!("International Standard Recording Code:    {}", optional_audio_tags.international_standard_recording_code.to_owned().unwrap());
        }
        else{
            println!("International Standard Recording Code:    {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.soft_hard_setting.is_some(){
            println!("Soft Hard Setting:                        {}", optional_audio_tags.soft_hard_setting.to_owned().unwrap());
        }
        else{
            println!("Soft Hard Setting:                        {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.year.is_some(){
            println!("Year:                                     {}", optional_audio_tags.year.to_owned().unwrap());
        }
        else{
            println!("Year:                                     {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.involved_people.is_some(){
            println!("Involved People:                          {}", optional_audio_tags.involved_people.to_owned().unwrap());
        }
        else{
            println!("Involved People:                          {}", NOT_AVAILABLE);  
        }
        if !&optional_audio_tags.commercial_info_url.is_empty(){
            for commercial_info_url in &optional_audio_tags.commercial_info_url {
            println!("Commercial Info URL:                      {:?}", *commercial_info_url);
            }
         }
         else{
            println!("Commercial Info URL:                      {}", NOT_AVAILABLE);
        }
        if optional_audio_tags.copyright_info_url.is_some(){
            println!("Copyright_Info URL:                       {:?}", &optional_audio_tags.copyright_info_url.as_ref());
        }
        else{
            println!("Copyright Info URL:                       {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.official_webpage.is_some(){
            println!("Official Webpage:                         {:?}", &optional_audio_tags.official_webpage.as_ref());
        }
        else{
            println!("Official Webpage:                         {}", NOT_AVAILABLE);  
        }
        if !&optional_audio_tags.official_artist_webpage.is_empty(){
            for official_artist_webpage in &optional_audio_tags.official_artist_webpage {
            println!("Official Artist Webpage:                  {:?}", *official_artist_webpage);
            }
         }
         else{
            println!("Official Artist Webpage:                  {}", NOT_AVAILABLE);
        }
        if optional_audio_tags.official_audio_source_webpage.is_some(){
            println!("Official Audio Source Webpage:            {:?}", &optional_audio_tags.official_audio_source_webpage.as_ref());
        }
        else{
            println!("Official Audio Source Webpage:            {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.official_internet_radio_webpage.is_some(){
            println!("Official Internet Radio Webpage:          {:?}", &optional_audio_tags.official_internet_radio_webpage.as_ref());
        }
        else{
            println!("Official Internet Radio Webpage:          {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.payment_url.is_some(){
            println!("Payment URL:                              {:?}", &optional_audio_tags.payment_url.as_ref());
        }
        else{
            println!("Payment URL:                              {}", NOT_AVAILABLE);  
        }
        if optional_audio_tags.publishers_official_webpage.is_some(){
            println!("Publishers Official Webpage:              {:?}", &optional_audio_tags.publishers_official_webpage.as_ref());
        }
        else{
            println!("Publishers Official Webpage:              {}", NOT_AVAILABLE);  
        } // End of inner if
    
    
      } // End of For
    
    } // End of if
    else
    {
        println!("\n\nOptional Informaion is Not Available.")
    }
    
}
*/

/// Prints Optional Information in MP3 file.
fn print_optional_information(mp3_meta_data: &MP3Metadata) {
    if !mp3_meta_data.optional_info.is_empty() {
        println!("\n========== OPTIONAL INFORMATION ==========");
        for optional_audio_tags in mp3_meta_data.optional_info[0..1].iter() {
            println!("Position:                                 {}", optional_audio_tags.position);
            println!("Major Version:                            {}", optional_audio_tags.major_version);
            println!("Minor Version:                            {}", optional_audio_tags.minor_version);


            match &optional_audio_tags.album_movie_show {
                Some(v) => println!("Album Movie Show:                         {}", v),
                None    => println!("Album Movie Show:                         {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.bpm {
                Some(v) => println!("Beats Per Minute:                         {}", v),
                None    => println!("Beats Per Minute:                         {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.composers.is_empty() {
                false => for composer in &optional_audio_tags.composers {
                    println!("Composer:                                 {}", composer);
                },
                true  => println!("Composer:                                 {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.content_type.is_empty() {
                false => for genre in &optional_audio_tags.content_type {
                    println!("Genre:                                    {:?}", genre);
                },
                true  => println!("Genre:                                    {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.copyright {
                Some(v) => println!("Copyright:                                {}", v),
                None    => println!("Copyright:                                {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.date {
                Some(v) => println!("Date:                                     {}", v),
                None    => println!("Date:                                     {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.playlist_delay {
                Some(v) => println!("Playlist Delay:                           {}", v),
                None    => println!("Playlist Delay:                           {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.encoded_by {
                Some(v) => println!("Encoded By:                               {}", v),
                None    => println!("Encoded By:                               {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.text_writers.is_empty() {
                false => for text_writers in &optional_audio_tags.text_writers {
                    println!("Text Writers:                             {}", text_writers);
                },
                true  => println!("Text Writers:                             {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.file_type {
                Some(v) => println!("File Type:                                {}", v),
                None    => println!("File Type:                                {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.time {
                Some(v) => println!("Time:                                     {}", v),
                None    => println!("Time:                                     {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.content_group_description {
                Some(v) => println!("Content Group Description:                {}", v),
                None    => println!("Content Group Description:                {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.subtitle_refinement_description {
                Some(v) => println!("Subtitle Refinement Description:          {}", v),
                None    => println!("Subtitle Refinement Description:          {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.title {
                Some(v) => println!("Title:                                    {}", v),
                None    => println!("Title:                                    {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.initial_key {
                Some(v) => println!("Initial Key:                              {}", v),
                None    => println!("Initial Key:                              {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.language {
                Some(v) => println!("Language:                                 {}", v),
                None    => println!("Language:                                 {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.length {
                Some(v) => println!("Length:                                   {}", v),
                None    => println!("Length:                                   {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.media_type {
                Some(v) => println!("Media Type:                               {}", v),
                None    => println!("Media Type:                               {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.original_album_move_show_title {
                Some(v) => println!("Original Album Movie Show Title:          {}", v),
                None    => println!("Original Album Movie Show Title:          {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.original_filename {
                Some(v) => println!("Original Filename:                        {}", v),
                None    => println!("Original Filename:                        {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.original_text_writers.is_empty() {
                false => for original_text_writers in &optional_audio_tags.original_text_writers {
                    println!("Original Text Writers:                    {}", original_text_writers);
                },
                true  => println!("Original Text Writers:                    {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.original_artists.is_empty() {
                false => for original_artists in &optional_audio_tags.original_artists {
                    println!("Original Artists:                         {}", original_artists);
                },
                true  => println!("Original Artists:                         {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.original_release_year {
                Some(v) => println!("Original Release Year:                    {}", v),
                None    => println!("Original Release Year:                    {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.file_owner {
                Some(v) => println!("File Owner:                               {}", v),
                None    => println!("File Owner:                               {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.performers.is_empty() {
                false => for performers in &optional_audio_tags.performers {
                    println!("Performers:                               {}", performers);
                },
                true  => println!("Performers:                               {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.band {
                Some(v) => println!("Band:                                     {}", v),
                None    => println!("Band:                                     {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.conductor {
                Some(v) => println!("Conductor:                                {}", v),
                None    => println!("Conductor:                                {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.interpreted {
                Some(v) => println!("Interpreted:                              {}", v),
                None    => println!("Interpreted:                              {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.part_of_a_set {
                Some(v) => println!("Part Of A Set:                            {}", v),
                None    => println!("Part Of A Set:                            {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.publisher {
                Some(v) => println!("Publisher:                                {}", v),
                None    => println!("Publisher:                                {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.track_number {
                Some(v) => println!("Track Number:                             {}", v),
                None    => println!("Track Number:                             {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.recording_dates {
                Some(v) => println!("Recording Dates:                          {}", v),
                None    => println!("Recording Dates:                          {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.internet_radio_station_name {
                Some(v) => println!("Internet Radio Station Name:              {}", v),
                None    => println!("Internet Radio Station Name:              {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.internet_radio_station_owner {
                Some(v) => println!("Internet Radio Station Owner:             {}", v),
                None    => println!("Internet Radio Station Owner:             {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.size {
                Some(v) => println!("Size:                                     {}", v),
                None    => println!("Size:                                     {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.international_standard_recording_code {
                Some(v) => println!("International Standard Recording Code:    {}", v),
                None    => println!("International Standard Recording Code:    {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.soft_hard_setting {
                Some(v) => println!("Soft Hard Setting:                        {}", v),
                None    => println!("Soft Hard Setting:                        {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.year {
                Some(v) => println!("Year:                                     {}", v),
                None    => println!("Year:                                     {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.involved_people {
                Some(v) => println!("Involved People:                          {}", v),
                None    => println!("Involved People:                          {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.commercial_info_url.is_empty() {
                false => for commercial_info_url in &optional_audio_tags.commercial_info_url {
                    println!("Commercial Info URL:                      {:?}", commercial_info_url);
                },
                true  => println!("Commercial Info URL:                      {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.copyright_info_url {
                Some(v) => println!("Copyright Info URL:                       {:?}", v),
                None    => println!("Copyright Info URL:                       {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.official_webpage {
                Some(v) => println!("Official Webpage:                         {:?}", v),
                None    => println!("Official Webpage:                         {}", NOT_AVAILABLE),
            }

            match optional_audio_tags.official_artist_webpage.is_empty() {
                false => for official_artist_webpage in &optional_audio_tags.official_artist_webpage {
                    println!("Official Artist Webpage:                  {:?}", official_artist_webpage);
                },
                true  => println!("Official Artist Webpage:                  {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.official_audio_source_webpage {
                Some(v) => println!("Official Audio Source Webpage:            {:?}", v),
                None    => println!("Official Audio Source Webpage:            {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.official_internet_radio_webpage {
                Some(v) => println!("Official Internet Radio Webpage:          {:?}", v),
                None    => println!("Official Internet Radio Webpage:          {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.payment_url {
                Some(v) => println!("Payment URL:                              {:?}", v),
                None    => println!("Payment URL:                              {}", NOT_AVAILABLE),
            }

            match &optional_audio_tags.publishers_official_webpage {
                Some(v) => println!("Publishers Official Webpage:              {:?}", v),
                None    => println!("Publishers Official Webpage:              {}", NOT_AVAILABLE),
            }
        }
    } else {
        println!("\n\nOptional Information is Not Available.");
    }
}

fn main() {
    let file = match env::args().skip(1).next() {
        Some(argument) => argument,
        None => {
            println!("\nPlease provide a mp3 filename.");
            return
        }
    };
    let mp3_meta_data = mp3_metadata::read_from_file(&file).expect("File error");
    
    
    println!("\nShowing MP3 MetaData for file : {}", &file);
    println!("______________________________________________________");

    print_tags(&mp3_meta_data);
    print_first_frame(&mp3_meta_data);
    print_optional_information(&mp3_meta_data);
   
    println!("\n========== END ==========");
     
}
