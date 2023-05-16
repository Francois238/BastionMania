import { Component, EventEmitter, Input, Output } from '@angular/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { AdminService } from '../admin.service';
import { UserInfo } from '../user-info';

@Component({
  selector: 'app-user-detail',
  templateUrl: './user-detail.component.html',
  styleUrls: ['./user-detail.component.scss']
})
export class UserDetailComponent {


  @Input() user!: UserInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  message : string =''

  constructor(protected adminService : AdminService, private modalService: NgbModal) { }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_user(this.user.id).subscribe({

      next: (data : number) => {
        
        this.message="L'administrateur a bien été supprimé"
        this.newItemEvent.emit("delete");

        
      },
      error: (e) => {
        
        this.message="Impossible de supprimer l'administrateur"
      },
    })

  }

}
