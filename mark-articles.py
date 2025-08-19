import markdown
import os
import sys;



def create_markdown(file_name):
    with open(file_name + '.md', 'r') as f:
        text = f.read()
        html = markdown.markdown(text)
        with open(file_name + '.html', 'w') as f:
            f.write(html)

if __name__ == "__main__":
    if len(sys.argv) < 1 or len(sys.argv) > 3:
        print("Invalid number of arguments")
    else:
        print("Valid number of arguments")
        print(sys.argv[0])
        print(sys.argv[1])
        for subdir, dirs, files in os.walk(sys.argv[1]):
            for file in files:
                #print os.path.join(subdir, file)
                filepath = subdir + file

                if filepath.endswith(".md"):
                    print (filepath)
                    print (os.path.splitext(filepath)[0])
                    create_markdown(os.path.splitext(filepath)[0])