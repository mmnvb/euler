import os

# Путь к папке, содержащей ваши папки от "1" до "100"
folder_path = "C:/Users/Bobosh/Desktop/euler/1"

# Пройдемся по всем папкам в указанной директории
for folder_name in os.listdir(folder_path):
    if os.path.isdir(os.path.join(folder_path, folder_name)):
        try:
            # Преобразуем имя папки в число и обратно в строку с ведущими нулями
            folder_number = int(folder_name)
            new_folder_name = f"{folder_number:03}"  # 3 знака, включая ведущие нули
            
            # Полный путь к текущей папке и новому имени папки
            old_folder_path = os.path.join(folder_path, folder_name)
            new_folder_path = os.path.join(folder_path, new_folder_name)
            
            # Переименование папки
            os.rename(old_folder_path, new_folder_path)
            print(f"Переименовано: {folder_name} -> {new_folder_name}")
        except ValueError:
            print(f"Не удалось обработать папку: {folder_name}")
