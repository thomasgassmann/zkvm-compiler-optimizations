def preprocess_input(input_file, output_file, threshold):
    with open(input_file, 'r') as file:
        lines = file.readlines()

    first_line = lines[0].strip().split()
    num_timetabled_trips = int(first_line[0])
    num_deadhead_trips = int(first_line[1])

    timetabled_trips = []
    for i in range(1, num_timetabled_trips + 1):
        if i <= threshold:
            timetabled_trips.append(lines[i].strip())

    deadhead_trips = []
    for i in range(num_timetabled_trips + 1, num_timetabled_trips + num_deadhead_trips + 1):
        if i - num_timetabled_trips <= threshold:
            deadhead_trips.append(lines[i].strip())

    with open(output_file, 'w') as file:
        file.write(f"{len(timetabled_trips)} {len(deadhead_trips)}\n")

        for trip in timetabled_trips:
            file.write(f"{trip}\n")

        for trip in deadhead_trips:
            file.write(f"{trip}\n")

input_file = 'inp.in'
output_file = 'inp_small.in'
threshold = 10
preprocess_input(input_file, output_file, threshold)
