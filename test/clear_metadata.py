import os
import subprocess
import shutil

ROOT_DIR = os.path.dirname(__file__)
DRY_RUN = False

def list_mdls(path):
    try:
        p = subprocess.run(['mdls', path], capture_output=True, check=True)
        return p.stdout.decode('utf-8', errors='replace')
    except Exception:
        return ''

def clear_xattrs(path):
    try:
        subprocess.run(['xattr', '-c', path], capture_output=True)
    except Exception:
        pass

def clear_docx(path):
    try:
        import docx
    except Exception:
        return False
    try:
        d = docx.Document(path)
    except Exception:
        return False
    cp = d.core_properties
    for k in ['title','subject','author','category','comments','keywords','last_modified_by']:
        try:
            setattr(cp, k, '')
        except Exception:
            pass
    if DRY_RUN:
        return True
    try:
        d.save(path)
        return True
    except Exception:
        return False

def clear_pdf(path):
    try:
        from pypdf import PdfReader, PdfWriter
    except Exception:
        return False
    try:
        r = PdfReader(path)
    except Exception:
        return False
    w = PdfWriter()
    for p in r.pages:
        w.add_page(p)
    try:
        try:
            w.remove_metadata()
        except Exception:
            w.add_metadata({})
        if DRY_RUN:
            return True
        with open(path, 'wb') as f:
            w.write(f)
        return True
    except Exception:
        return False

def clear_with_exiftool(path):
    if shutil.which('exiftool') is None:
        return False
    cmd = ['exiftool', '-overwrite_original', '-all=', path]
    if DRY_RUN:
        return True
    try:
        subprocess.run(cmd, capture_output=True, check=True)
        return True
    except Exception:
        return False

def handle_file(path):
    print(f"=================")
    print(f"文件：{path}")
    info = list_mdls(path)
    if info:
        print("详细信息：")
        print(info)
    ext = os.path.splitext(path)[1].lower()
    ok = False
    if ext == '.docx':
        ok = clear_docx(path)
    elif ext == '.pdf':
        ok = clear_pdf(path)
    else:
        ok = clear_with_exiftool(path)
    clear_xattrs(path)
    print(f"清空结果：{'成功' if ok else '尝试清空（可能部分失败）'}")

def main():
    for dirpath, _, filenames in os.walk(ROOT_DIR):
        for fn in filenames:
            handle_file(os.path.join(dirpath, fn))

if __name__ == '__main__':
    main()
